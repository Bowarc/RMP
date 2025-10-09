#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Command {
    GetLibrary,
    Player(PlayerCommand),
    Downloader(DownloaderCommand),
    Playlist(PlaylistCommand),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlayerCommand {
    Play,
    Pause,
    GetPlayState,

    GetCurrentlyPlaying,

    AddToQueue(uuid::Uuid),
    RemoveFromQueue(u16),
    GetQueue,
    ClearQueue,

    SetVolume(f32),
    GetVolume,

    SetPosition(std::time::Duration),
    GetPosition,

    // Skip(u8), // Skip x songs
    SetDeviceByName(String),
    GetDeviceName,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum DownloaderCommand {
    QueueDownload(String),
    StopCurrentDownload,
    FetchCurrent,
    FetchQueue,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum PlaylistCommand {
    GetAll,
    GetOne(uuid::Uuid), // The id of the playlist
    Create(crate::playlist::Playlist),
    Delete(uuid::Uuid),
    Rename {
        playlist_uuid: uuid::Uuid,
        new_name: String,
    },
    AddToPlaylist {
        playlist_uuid: uuid::Uuid,
        song_uuid: uuid::Uuid,
    },
    RemoveFromPlaylist {
        playlist_uuid: uuid::Uuid,
        song_index: u16, // That's 65k, it should be enough
    },
}

impl From<PlayerCommand> for Command {
    fn from(pc: PlayerCommand) -> Self {
        Command::Player(pc)
    }
}

impl From<DownloaderCommand> for Command {
    fn from(dc: DownloaderCommand) -> Self {
        Command::Downloader(dc)
    }
}

impl From<PlaylistCommand> for Command {
    fn from(pc: PlaylistCommand) -> Self {
        Command::Playlist(pc)
    }
}
