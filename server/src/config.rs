pub struct PlayerConfig {
    song_path: std::path::PathBuf,
    volume: f32,
}

impl PlayerConfig {
    pub fn song_path(&self) -> &std::path::PathBuf {
        &self.song_path
    }
    pub fn volume(&self) -> f32 {
        self.volume
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            // song_path: PathBuf::from("./songs/"),
            song_path: shared::path::songs_path(),
            volume: 0.01,
        }
    }
}
