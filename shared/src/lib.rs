#[macro_use]
extern crate log;

pub mod command;
pub mod server;
pub mod client;
pub mod message;
pub mod song;
pub mod audio_device_utils;
pub mod path;


pub const DEFAULT_ADDRESS: std::net::SocketAddr = std::net::SocketAddr::V4(
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 19864),
);
