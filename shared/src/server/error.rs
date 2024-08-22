#[derive(Debug, thiserror::Error, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Error {
    #[error("This is a test error and shouldn't be used in any public context")]
    Test,

    #[error(transparent)]
    PlayerError(#[from] PlayerError),

    #[error("{0}")]
    SocketError(String),
}

#[derive(Debug, thiserror::Error, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlayerError {
    #[error("This is a test error and shouldn't be used in any public context")]
    Test,

    #[error("The current stack is empty")]
    EmptyStack,

    #[error("{name} player failled to initialize due to: {error}")]
    Initialisation { name: String, error: String },

    #[error("Could not read file: {target} due to: {error}")]
    FileRead { target: String, error: String },

    #[error("Could not set the device to {device} due to: {e}")]
    SetDeviceError { device: String, e: String },

    #[error("Could not seek to the requested time due to: {0}")]
    SeekError(String),

    #[error("{name} player found an error while using it's audio device: {e}")]
    DeviceError { name: String, e: String },
}
