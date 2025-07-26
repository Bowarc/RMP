mod rodio_player;
pub use rodio_player::RodioPlayer as PlayerImpl;

pub type Result<T> = std::result::Result<T, crate::error::PlayerError>;

pub trait Player {
    fn play(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;

    fn currently_playing(&self) -> Result<shared::song::Song> {
        let q = self.queue()?;

        let index = self.currently_playing_index()?;

        if index >= q.len() as u64 {
            // self.pause()?;
            // self.clear_queue()?;
            return Err(crate::error::PlayerError::Custom(String::from(
                "An error occured while validating the queue: currently_playing_index was ouside the queue's range",
            )));
        };

        Ok(q.get(index as usize).unwrap().clone()) // This unwrap is fine as we just checked if the index was inbounds
    }
    fn currently_playing_index(&self) -> Result<u64>;

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
