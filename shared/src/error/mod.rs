pub mod client;
pub mod server;


#[derive(Debug, thiserror::Error)]
pub enum ImporterError{
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Ron(#[from] ron::Error)
}


