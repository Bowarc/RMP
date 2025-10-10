#[macro_use(trace, debug, info, error)]
extern crate log;

mod data;

pub struct Client {
    socket: shared::Socket,
    player_data: data::MusicPlayerData,
    downloader_data: data::DownloaderData,
    playlist_data: data::PlaylistData,
    ping: std::time::Duration,
}

impl Client {
    pub fn init() -> Self {
        let mut socket = shared::socket::new_nonblocking().unwrap();

        socket.send(shared::message::ClientMessage::Ping).unwrap();
        let ping_base = std::time::Instant::now();
        debug!("Pinging");
        loop {
            match socket.try_recv() {
                Ok((_h, shared::message::ServerMessage::Pong)) => break,
                Ok((_h, msg)) => {
                    info!("Ingored message: {msg:?}, received while ping checking");
                }
                Err(e) => {
                    if let shared::error::SocketError::StreamRead(ref io) = e
                        && io.kind() == std::io::ErrorKind::WouldBlock
                    {
                        continue;
                    }
                    error!("{e}")
                }
            }
        }
        let ping = ping_base.elapsed();
        debug!("Estimated ping: {:.3}ms", ping.as_secs_f32() / 1000.);

        Self {
            socket,
            player_data: data::MusicPlayerData::default(),
            downloader_data: data::DownloaderData::default(),
            playlist_data: data::PlaylistData::default(),
            ping,
        }
    }

    pub fn update(&mut self) -> Result<usize, shared::error::SocketError> {
        use shared::error::SocketError;
        let mut message_count = 0;
        loop {
            match self.socket.try_recv() {
                Ok((_h, m)) => {
                    message_count += 1;
                    if let Err(e) = self.handle_server_message(m) {
                        error!("{e}");
                        break;
                    }
                }
                Err(e) => {
                    if let SocketError::StreamRead(ref io) = e
                        && io.kind() == std::io::ErrorKind::WouldBlock
                    {
                        break;
                    }
                    error!("{e}");

                    return Err(e);
                }
            }
        }
        Ok(message_count)
    }
    pub fn handle_server_message(
        &mut self,
        message: shared::message::ServerMessage,
    ) -> Result<Option<shared::error::server::Error>, shared::error::SocketError> {
        use shared::message::{ClientMessage, ServerMessage};
        if message != ServerMessage::Ping {
            debug!("Received message: {message:?}");
        }
        match message {
            ServerMessage::Ping => {
                self.socket.send(ClientMessage::Pong)?;
            }
            ServerMessage::Exit => todo!(),
            ServerMessage::PlayerStatePause => self.player_data.playing = false,
            ServerMessage::PlayerStatePlay => self.player_data.playing = true,
            ServerMessage::CurrentlyPlaying { song, .. } => {
                debug!("Setting current song");
                self.player_data.current_song = Some(song.clone())
            }
            ServerMessage::PlayerVolume(v) => self.player_data.volume = v,
            ServerMessage::PlayerQueue(queue) => self.player_data.song_queue = queue,
            ServerMessage::AudioDevice(device_name) => {
                self.player_data.current_device = device_name
            }
            ServerMessage::Position(position) => self.player_data.position = position,
            ServerMessage::Library(songs) => self.player_data.song_list = songs,

            ServerMessage::CurrentDownloads(reports) => {
                self.downloader_data.current_downloads = reports
            }
            ServerMessage::Playlists(all) => self.playlist_data.all = all,

            ServerMessage::Error(error) => return Ok(Some(error)),

            _ => debug!("Unhandled message: {message:?}"),
        }
        Ok(None)
    }

    pub fn send_multiple(
        &mut self,
        messages: Vec<shared::message::ClientMessage>,
    ) -> Result<(), shared::error::SocketError> {
        let socket = self.socket_mut();
        for msg in messages.into_iter() {
            trace!("Sending: {msg:?}");
            socket.send(msg)?;
        }

        Ok(())
    }
}

impl Client {
    pub fn socket(&self) -> &shared::Socket {
        &self.socket
    }
    pub fn socket_mut(&mut self) -> &mut shared::Socket {
        &mut self.socket
    }

    pub fn player_data(&self) -> &data::MusicPlayerData {
        &self.player_data
    }
    pub fn player_data_mut(&mut self) -> &mut data::MusicPlayerData {
        &mut self.player_data
    }
    pub fn downloader_data(&self) -> &data::DownloaderData {
        &self.downloader_data
    }
    pub fn downloader_data_mut(&mut self) -> &mut data::DownloaderData {
        &mut self.downloader_data
    }
    pub fn playlist_data(&self) -> &data::PlaylistData {
        &self.playlist_data
    }
    pub fn playlist_data_mut(&mut self) -> &mut data::PlaylistData {
        &mut self.playlist_data
    }
    pub fn ping(&self) -> &std::time::Duration {
        &self.ping
    }

    pub fn exit(&mut self) {
        if let Err(e) = self.socket.send(shared::message::ClientMessage::Exit) {
            error!("Failed to send exit message to server due to: {e}");
        }
    }
}

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             socket: shared::socket::new_nonblocking().unwrap(),
//             player_data: data::MusicPlayerData::default(),
//             downloader_data: data::DownloaderData::default(),
//         }
//     }
// }
