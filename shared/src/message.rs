#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ClientMessage {


    Ping,
    Pong,

    Command(crate::command::Command),
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServerMessage {
    Ping,
    Pong,

    // player
    PlayerStatePause,
    PlayerStatePlay, // could use an enum w/ PlayerStateChanged but it might be overkill

    PlayerVolume(f32), // returned also by setvolume (will make client synchronisation easier)

    PlayerQueueUpdated(Vec<crate::song::Song>),





    // downloader

    Error(crate::server::error::Error),
}

impl networking::Message for ClientMessage {
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }

    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_ping() -> Self {
        Self::Ping
    }

    fn default_pong() -> Self {
        Self::Pong
    }
}

impl networking::Message for ServerMessage {
    fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }

    fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    fn default_ping() -> Self {
        Self::Ping
    }

    fn default_pong() -> Self {
        Self::Pong
    }
}
