pub struct PlayerConfig {
    song_path: std::path::PathBuf,
    volume: f32
}

impl PlayerConfig {
    pub fn song_path(&self) -> &std::path::PathBuf {
        &self.song_path
    }
    pub fn volume(&self) -> f32{
        self.volume
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        use std::path::PathBuf;
        PlayerConfig {
            song_path: PathBuf::from("./songs/"),
            volume: 0.01
        }
    }
}