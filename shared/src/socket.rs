pub const DEFAULT_ADDRESS: std::net::SocketAddr = std::net::SocketAddr::V4(
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(127, 0, 0, 1), 19864),
);

pub type Socket =
    networking::socket::Socket<crate::message::ServerMessage, crate::message::ClientMessage>;

pub fn new() -> Result<Socket, crate::error::client::SocketError> {
    use {crate::error::client::SocketError, networking::Socket, std::net::TcpStream};

    Ok(Socket::new(
        TcpStream::connect(DEFAULT_ADDRESS)
            .map_err(|e| SocketError::Initialisation(e.to_string()))?,
    ))
}
pub fn new_nonblocking() -> Result<Socket, crate::error::client::SocketError> {
    use {crate::error::client::SocketError, networking::Socket, std::net::TcpStream};
    let stream = TcpStream::connect(DEFAULT_ADDRESS)
        .map_err(|e| SocketError::Initialisation(e.to_string()))?;

    stream
        .set_nonblocking(true)
        .map_err(|e| SocketError::Initialisation(e.to_string()))?;

    Ok(Socket::new(stream))
}
