use std::net::Ipv4Addr;

use crate::{Data};




pub fn check_ip_validity(data: &Data) -> bool
{
    data.ip_addr.parse::<Ipv4Addr>().is_ok()
}
