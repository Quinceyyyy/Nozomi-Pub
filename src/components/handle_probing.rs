use std::time::Duration;
use std::borrow::Cow;
use futures::future::join_all;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::timeout;
use tokio::{self, net::TcpStream};
use std::error::Error;

use crate::{Data};


async fn http_probing(ip: &String, port: u16) -> bool
{
    let socket = format!("{}:{}", ip, port);
    let http_req = format!(
    "GET / HTTP/1.1\r\n\
     Host: {}\r\n\
     User-Agent: Mozilla/5.0\r\n\
     Accept: */*\r\n\
     Connection: close\r\n\r\n", 
    ip
    );
    let mut buffer = [0; 1024];

    match TcpStream::connect(&socket).await {
        Ok(mut stream) => {
            if stream.write_all(http_req.as_bytes()).await.is_ok() {
                if let Ok(Ok(reader)) = timeout(Duration::from_secs(10), stream.read(&mut buffer)).await {
                    if reader > 0 {
                        println!("Port: {} -> HTTP", port);
                        return true;
                    }
                }
            }
            return false;
        }
        Err(_) => false,
    }
}


async fn service_type(port: u16, response: Cow<'_,str>)
{
    if response.contains("SSH") {
        println!("Port: {} -> SSH", port);
    } else if response.contains("FTP") {
        println!("Port: {} -> FTP", port);
    } else {
        println!("Couldnt recognise service on port: {}", port);
    }
}

async fn probing_task(ip: String, port: u16) -> Result<(), Box<dyn Error>>
{
    let ip_addr = format!("{}:{}", ip, port);
    let mut buffer = [0; 1024];

    if port == 80 {
        if !http_probing(&ip, port).await {
            println!("Probing failed on port {}", port);
        }
        return Ok(());
    }

    match TcpStream::connect(&ip_addr).await {
        Ok(mut stream) => {
        match timeout(Duration::from_secs(10),  stream.read(&mut buffer)).await {
            Ok(Ok(reader)) if reader > 0 => {
                    let response = String::from_utf8_lossy(&buffer[..reader]);
                    service_type(port, response).await;
                }
                Ok(Ok(0)) => {println!("No banner reseved on port: {}", port);}
                Ok(_) => {println!("Unknown service on port: {}", port);}
                Err(_) => {println!("ip: {} port: {} timeout", ip, port)}
            }
        }
        Err(_) => {println!("port: {} couldn't connect", port);}
    }
    Ok(())
}

pub async fn probe_ports(data: &mut Data) -> Result<(), Box<dyn Error>> 
{
    let ip = data.ip_addr.clone();

    let tasks = data.open_ports.iter().map(|&port| {
        let ip = ip.clone();
        tokio::spawn(async move {
            if let Err(e) = probing_task(ip, port).await {
                eprintln!("Error when probing port: {} error: {}", port, e);
            }
        })
    });
    join_all(tasks).await;

    Ok(())
}
