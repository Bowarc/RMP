#[derive(Debug, thiserror::Error, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Error {
    #[error("This is a test error and shouldn't be used in any public context")]
    Test,

    #[error(transparent)]
    Player(#[from] PlayerError),

    #[error("{0}")]
    Socket(String),

    #[error(transparent)]
    Downloader(#[from] DownloaderError),

    #[error("{0}")]
    Io(String)

    
}

#[derive(Debug, thiserror::Error, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlayerError {
    #[error("This is a test error and shouldn't be used in any public context")]
    Test,

    #[error("The player is not currently playing")]
    NotPlaying,

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

    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, thiserror::Error, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum DownloaderError{
    #[error("This is a test error and shouldn't be used in any public context")]
    Test,
    #[error("Failed to parse '{url}' due to: {reason}")]
    UrlParse{
        url: String,
        reason: String,
    },
    #[error("The provider '{0}' is not supported")]
    ProviderNotSupported(String),
    #[error("Playlists are not currently supported")]
    PlaylistNotSupported
}
