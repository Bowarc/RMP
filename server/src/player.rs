mod rodio_player;
pub use rodio_player::RodioPlayer as PlayerImpl;

pub type Result<T> = std::result::Result<T, crate::error::PlayerError>;

pub trait Player {
    fn play(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;

    fn add_queue(&mut self, uuid: uuid::Uuid) -> Result<()>;
    fn remove_queue(&mut self, uuid: uuid::Uuid) -> Result<()>;
    fn clear_queue(&mut self) -> Result<()>;
    fn queue(&self) -> Result<Vec<shared::song::Song>>; // TODO: swap to a user friendly song identification system

    fn set_volume(&mut self, val: f32) -> Result<()>;
    fn volume(&self) -> Result<f32>;

    fn audio_device(&self) -> Result<String>;
    fn set_device_by_name(&mut self, new_device_name: &str) -> Result<()>;

    fn set_pos(&mut self, pos: std::time::Duration) -> Result<()>;
    fn pos(&self) -> Result<std::time::Duration>;

    fn update(&mut self) -> Result<()>;
}

// pub type playlist = Vec<uuid::Uuid>;

