mod rodio_player;

pub trait Player {
    fn play(&mut self) -> Result<(), crate::error::PlayerError>;
    fn pause(&mut self) -> Result<(), crate::error::PlayerError>;

    fn add_queue(&mut self, uuid: uuid::Uuid) -> Result<(), crate::error::PlayerError>;
    fn remove_queue(&mut self, uuid: uuid::Uuid) -> Result<(), crate::error::PlayerError>;

    fn update(&mut self) -> Result<(), crate::error::PlayerError>;
}

pub type playlist = Vec<uuid::Uuid>;

// pub struct Song {
//     uuid: uuid::Uuid,
//     name: String,
//     duration: std::time::Duration,
//     // author: Option<String>
// }
