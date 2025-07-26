#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ClientMessage {
    Ping,
    Pong,
    Exit,

    Command(crate::command::Command),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServerMessage {
    Ping,
    Pong,
    Exit,

    // player
    PlayerStatePause,
    PlayerStatePlay, // could use an enum w/ PlayerStateChanged but it might be overkill

    CurrentlyPlaying { song: crate::song::Song, index: u64 },

    PlayerVolume(f32), // returned also by setvolume (will make client synchronisation easier)

    PlayerQueue(Vec<crate::song::Song>),

    AudioDevice(String),

    Position(std::time::Duration),

    // downloader
    Error(crate::error::server::Error),
}

impl networking::Message for ClientMessage {
    fn is_exit(&self) -> bool {
        matches!(self, Self::Exit)
    }
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }

    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_exit() -> Self {
        Self::Exit
    }
    fn default_ping() -> Self {
        Self::Ping
    }

    fn default_pong() -> Self {
        Self::Pong
    }
}

impl networking::Message for ServerMessage {
    fn is_exit(&self) -> bool {
        matches!(self, Self::Exit)
    }
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }
    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_exit() -> Self {
        Self::Exit
    }
    fn default_ping() -> Self {
        Self::Ping
    }
    fn default_pong() -> Self {
        Self::Pong
    }
}

impl From<super::command::PlayerCommand> for ClientMessage {
    fn from(pc: super::command::PlayerCommand) -> ClientMessage {
        use super::command::Command;
        ClientMessage::Command(Command::Player(pc))
    }
}

impl From<super::command::DownloaderCommand> for ClientMessage {
    fn from(pc: super::command::DownloaderCommand) -> ClientMessage {
        use super::command::Command;
        ClientMessage::Command(Command::Downloader(pc))
    }
}
