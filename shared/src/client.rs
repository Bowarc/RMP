pub struct Client {
    socket:
        networking::socket::Socket<crate::message::ServerMessage, crate::message::ClientMessage>,
}

impl Client {
    pub fn new() -> Result<Self, crate::error::client::SocketError> {
        use {crate::error::client::SocketError, networking::Socket, std::net::TcpStream};

        Ok(Self {
            socket: Socket::new(
                TcpStream::connect(crate::DEFAULT_ADDRESS)
                    .map_err(|e| SocketError::Initialisation(e.to_string()))?,
            ),
        })
    }
}

impl std::ops::Deref for Client {
    type Target =
        networking::socket::Socket<crate::message::ServerMessage, crate::message::ClientMessage>;

    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl std::ops::DerefMut for Client {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.socket
    }
}
