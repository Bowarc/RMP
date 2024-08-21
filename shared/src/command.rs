#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Command {
    Player(PlayerCommand),
    Downloader(DownloaderCommand),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlayerCommand {
    Play,
    Pause,

    AddToQueue(uuid::Uuid),
    RemoveFromQueue(uuid::Uuid),
    ClearQueue,

    SetVolume(f32),
    GetVolume,

    SetPosition(std::time::Duration),
    GetPosition,

    SetDeviceName(String)
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum DownloaderCommand {
    StartDownload(String),
    StopCurrentDownload,
    FetchCurrent,
    FetchQueue,
}


impl From<PlayerCommand> for Command{
    fn from(pc: PlayerCommand) -> Self {
        Command::Player(pc)
    }
}

impl From<DownloaderCommand> for Command{
    fn from(dc: DownloaderCommand) -> Self {
        Command::Downloader(dc)
    }
}