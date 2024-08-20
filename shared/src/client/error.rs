#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error("This is a test error and shouldn't be used in any public context")]
    Test,


}

#[derive(Debug, thiserror::Error)] // the server doens't need to care about client errors
pub enum SocketError{
    #[error("Could not initliaze the socket due to: {0}")]
    Initialisation(String),

    #[error(transparent)]
    Inner(#[from] networking::socket::SocketError)
}
