use std::net::{IpAddr, Ipv4Addr};

pub fn check_connection() -> bool{
    let result = ping::ping(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)) , None, None, None, None, None);
    return result.is_ok();
}