#[macro_use(debug, error)]
extern crate log;

mod data;

pub struct App {
    socket: shared::Socket,
    player_data: data::MusicPlayerData,
    downloader_data: data::DownloaderData,
}

impl App {
    pub fn update(&mut self) -> usize {
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
                    error!("{e}")
                }
            }
        }
        message_count
    }
    pub fn handle_server_message(
        &mut self,
        message: shared::message::ServerMessage,
    ) -> Result<Option<shared::error::server::Error>, shared::error::SocketError> {
        use shared::message::{ClientMessage, ServerMessage};
        debug!("Received message: {message:?}");
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
            ServerMessage::Error(error) => return Ok(Some(error)),

            _ => debug!("Unhandled message: {message:?}"),
        }
        Ok(None)
    }
}

impl App {
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
    pub fn exit(&mut self) {
        if let Err(e) = self.socket.send(shared::message::ClientMessage::Exit) {
            error!("Failed to send exit message to server due to: {e}");
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            socket: shared::socket::new_nonblocking().unwrap(),
            player_data: data::MusicPlayerData::default(),
            downloader_data: data::DownloaderData::default(),
        }
    }
}
