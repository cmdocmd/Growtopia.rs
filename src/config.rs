use std::net::Ipv4Addr;

pub struct Config {
  pub host: Ipv4Addr,
  pub port: u16
}

pub fn get() -> Config {
  Config {
    host: Ipv4Addr::new(0, 0, 0, 0),
    port: 17091
  }
}