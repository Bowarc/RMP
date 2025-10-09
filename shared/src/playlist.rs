#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, std::cmp::PartialEq)]
pub struct Playlist {
    uuid: uuid::Uuid,
    name: String,
    songs: Vec<crate::song::Song>,
}

#[cfg(feature = "server")]
impl Playlist {
    // pub fn new(name: String) -> Self {
    //     Self {
    //         uuid: uuid::Uuid::new_v4(),
    //         name,
    //         songs: Vec::new(),
    //     }
    // }
    pub fn load(
        uuid: uuid::Uuid,
        song_dir_path: &std::path::Path,
    ) -> Result<Playlist, crate::error::server::PlaylistError> {
        use {crate::error::server::PlaylistError, std::fs::OpenOptions};

        let playlist_path = song_dir_path.join(format!("{uuid}.playlist"));

        let file = OpenOptions::new()
            .read(true)
            .open(playlist_path.clone())
            .map_err(|e| PlaylistError::FileRead {
                target: playlist_path.display().to_string(),
                error: e.to_string(),
            })?;

        let playlist = ron::de::from_reader(file).map_err(|e| PlaylistError::FileRead {
            target: playlist_path.display().to_string(),
            error: e.to_string(),
        })?;

        Ok(playlist)
    }

    pub fn save(
        &self,
        song_dir_path: &std::path::Path,
    ) -> Result<(), crate::error::server::PlaylistError> {
        use {crate::error::server::PlaylistError, std::fs::OpenOptions};

        let playlist_path = song_dir_path.join(format!("{uuid}.playlist", uuid = self.uuid));

        let file = OpenOptions::new()
            .create(true) // Overwrite if exists
            .write(true)
            .truncate(true)
            .open(playlist_path.clone())
            .map_err(|e| PlaylistError::FileRead {
                target: playlist_path.display().to_string(),
                error: e.to_string(),
            })?;

        ron::ser::to_writer(file, &self).map_err(|e| PlaylistError::FileWrite {
            target: playlist_path.display().to_string(),
            error: e.to_string(),
        })?;

        Ok(())
    }

    pub fn delete(
        uuid: uuid::Uuid,
        song_dir_path: &std::path::Path,
    ) -> Result<(), crate::error::server::PlaylistError> {
        use crate::error::server::PlaylistError;

        let playlist_path = song_dir_path.join(format!("{uuid}.playlist"));

        std::fs::remove_file(&playlist_path).map_err(|e| PlaylistError::FileDelete {
            target: playlist_path.display().to_string(),
            error: e.to_string(),
        })?;

        Ok(())
    }
}

impl Playlist {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            name: name.into(),
            songs: Default::default(),
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn uuid(&self) -> &uuid::Uuid {
        &self.uuid
    }
    pub fn songs(&self) -> &[crate::song::Song] {
        &self.songs
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn songs_mut(&mut self) -> &mut Vec<crate::song::Song> {
        &mut self.songs
    }
}
