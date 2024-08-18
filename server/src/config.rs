pub struct PlayerConfig{
    song_path: std::path::PathBuf
}

impl PlayerConfig{
    pub fn song_path(&self) -> &std::path::PathBuf{
        &self.song_path
    }
}

impl Default for PlayerConfig{
    fn default() -> Self {
        use std::path::PathBuf;
        PlayerConfig { song_path: PathBuf::from("./songs/")  }
    }
}