use tokio::{self, net::TcpStream};
use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::Semaphore;

use crate::{Data, END_PORT, START_PORT};



pub async fn scan_ip(data: &mut Data) -> Result<(), Box<dyn std::error::Error>>
{
    let mut scans= Vec::new();
    let semaphore = Arc::new(Semaphore::new(100));

    for port in START_PORT..=END_PORT {
        let ip = data.ip_addr.to_string();
        let semaphore = semaphore.clone();
        let scan = tokio::spawn(async move {

            let _permit = semaphore.acquire_owned().await.unwrap();
            if TcpStream::connect((ip.as_str(), port)).await.is_ok() {
                Some(port)
            } else {
                None
            }
        });
        scans.push(scan);
    }

    let total_ports = join_all(scans).await;

    data.open_ports = total_ports
        .into_iter()
        .filter_map(|res| res.ok().flatten())
        .collect();
    Ok(())  
}
