pub mod client;
pub mod server;

pub use networking::socket::SocketError;

#[derive(Debug, thiserror::Error)]
pub enum ImporterError{
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Ron(#[from] ron::Error)
}


