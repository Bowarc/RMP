#[derive(Default)]
pub struct MusicPlayerData {
    pub current_song: Option<shared::song::Song>,
    pub playing: bool,
    pub position: std::time::Duration,
    pub song_queue: Vec<shared::song::Song>,
    pub song_list: Vec<shared::song::Song>,
    pub volume: f32,

    pub current_device: String,
}

#[derive(Default)]
pub struct DownloaderData {
    pub new_download_url: String,
    pub current_downloads: Vec<shared::download::Report>, // Add a struct containing download reports, this isn't done in server atm
}

#[derive(Default)]
pub struct PlaylistData {
    pub all: Vec<shared::playlist::Playlist>,
    pub current: Option<uuid::Uuid>,
}

impl PlaylistData {
    pub fn get_current(&self) -> Option<&shared::playlist::Playlist> {
        let current = self.current.as_ref()?;
        self.get(current)
    }
    pub fn get(&self, uuid: &uuid::Uuid) -> Option<&shared::playlist::Playlist> {
        self.all.iter().find(|p| p.uuid() == uuid)
    }
}
