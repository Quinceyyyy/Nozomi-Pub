use tokio;
use std::env;

mod components;

use components:: {
    handle_ip,
    handle_probing,
    handle_scan,
    help,
};

pub const START_PORT: u16 = 1;
pub const END_PORT: u16 = 1024;

#[derive(Debug, Default)]
struct Data {
    ip_addr: String,
    open_ports: Vec<u16>,
}


#[tokio:: main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3{
        help::help_msg();
        return Ok(());
    }

    let mut data = Data::default();
    data.ip_addr = String::from(&args[1]);

    if !handle_ip::check_ip_validity(&data){
        println!("{} is not a valide IP", data.ip_addr);
        return Ok(());
    }

    handle_scan::scan_ip(&mut data).await?;
    handle_probing::probe_ports(&mut data).await?;

    Ok(())
}
