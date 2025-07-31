#[derive(Default)]
pub struct MusicPlayerData{
    pub current_song: Option<shared::song::Song>,
    pub playing: bool,
    pub position: std::time::Duration,
    pub song_queue: Vec<shared::song::Song>,
    pub song_list: Vec<shared::song::Song>,
    pub volume: f32,

    pub current_device: String
}

#[derive(Default)]
pub struct DownloaderData{
    pub new_download_url: String,
    pub current_downloads: Vec<()>, // Add a struct containing download reports, this isn't done in server atm
    
}
