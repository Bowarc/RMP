// Not a big fan of oop but let's try and we'll see later it that causes a problem
// Wasn't long lmao

use std::str::FromStr;

fn client_timeout(
    msg_received_this_tick: bool,

    socket_opt: &mut Option<
        networking::Socket<shared::message::ClientMessage, shared::message::ServerMessage>,
    >,
) {
    static mut LAST_RECEIVED_TIME: Option<std::time::Instant> = None;
    const MAX_SILENT_TIME: std::time::Duration = std::time::Duration::from_secs(1);
    const MAX_PING: std::time::Duration = std::time::Duration::from_secs(1);
    static mut PING_SENT: bool = false;

    if msg_received_this_tick {
        unsafe {
            // SAFETY: Absolutely no safety here, but since i don't use threads for now, it's good

            LAST_RECEIVED_TIME = Some(std::time::Instant::now());
            PING_SENT = false;
        }
    }

    // SAFETY: tkt
    #[allow(static_mut_refs)]
    if unsafe { LAST_RECEIVED_TIME.is_none() } {
        unsafe { LAST_RECEIVED_TIME = Some(std::time::Instant::now()) };
        return;
    }

    if unsafe { !PING_SENT } && unsafe { LAST_RECEIVED_TIME.unwrap().elapsed() > MAX_SILENT_TIME } {
        // debug!("Havn't heard from the client in a while, let's ping it");
        let _ = socket_opt
            .as_mut()
            .unwrap()
            .send(shared::message::ServerMessage::Ping);
        unsafe { PING_SENT = true };
        return;
    }

    if unsafe { PING_SENT }
        && unsafe { LAST_RECEIVED_TIME.unwrap().elapsed() > MAX_PING + MAX_SILENT_TIME }
    {
        warn!("Client did not respond to the ping, cutting the wire");
        let mut s = socket_opt.take().unwrap();
        let _ = s.send(shared::message::ServerMessage::Exit);
        unsafe { LAST_RECEIVED_TIME = None };
    }
}

pub fn recv_commands(
    socket_opt: &mut Option<
        networking::Socket<shared::message::ClientMessage, shared::message::ServerMessage>,
    >,
    music_player: &mut Box<dyn crate::player::Player>,
    download_mgr: &mut crate::downloader::DownloadManager,
) {
    use {
        // networking::proxy::ProxyMessage,
        crate::error::Error,
        shared::{
            command::Command,
            message::{ClientMessage, ServerMessage},
        },
    };

    let Some(socket) = socket_opt else {
        return;
    };

    let mut msg_received_this_tick = false;
    loop {
        let (_header, cm) = match socket.try_recv() {
            Ok(h_m) => h_m,
            Err(e) => {
                if let shared::error::SocketError::StreamRead(ref io) = e
                    && io.kind() == std::io::ErrorKind::WouldBlock
                {
                    break;
                }
                warn!("Client disconnected: ({e})");
                *socket_opt = None;
                return;
                // break;
            }
        };

        msg_received_this_tick = true;

        if cm != ClientMessage::Pong {
            info!("Received a message from socket: {cm:?}");
        }

        match cm {
            ClientMessage::Command(command) => match command {
                Command::Player(pc) => {
                    if let Err(e) = handle_player_command(socket, music_player, pc) {
                        let _ = socket.send(ServerMessage::Error(Error::Player(e)));
                    }
                }
                Command::Downloader(dc) => handle_downloader_command(socket, download_mgr, dc),
                Command::Playlist(pc) => handle_playlist_command(socket, pc),
                Command::GetLibrary => {
                    let songs_path = crate::config::songs_path();

                    let read_dir = match std::fs::read_dir(songs_path) {
                        Ok(rd) => rd,
                        Err(e) => {
                            error!("Failed to read songs dir due to: {e}");
                            if let Err(e) = socket.send(ServerMessage::Error(
                                shared::error::server::Error::Io(e.to_string()),
                            )) {
                                error!("Failed to send error to client due to: {e}");
                            }
                            return;
                        }
                    };

                    let songs = read_dir
                        .into_iter()
                        .flatten()
                        .filter(|item| {
                            item.file_type().is_ok() && item.file_type().unwrap().is_file()
                        })
                        .filter(|item| item.file_name().to_string_lossy().ends_with(".metadata"))
                        .flat_map(|item| {
                            shared::song::Song::load(
                                uuid::Uuid::from_str(
                                    &item.file_name().to_string_lossy().replace(".metadata", ""),
                                )
                                .unwrap(),
                                &crate::config::songs_path(),
                            )
                        })
                        .collect::<Vec<_>>();

                    debug!("{songs:?}");
                    if let Err(e) = socket.send(ServerMessage::Library(songs)) {
                        error!("Failed to send library to client due to: {e}");
                        return;
                    }
                }
            },
            ClientMessage::Ping => {
                if let Err(e) = socket.send(ServerMessage::Pong) {
                    error!("Failed to send back pong message to client due to: {e}",);
                }
            }
            ClientMessage::Pong => (),
            ClientMessage::Exit => unreachable!(),
        }
    }

    client_timeout(msg_received_this_tick, socket_opt);
}

pub fn handle_player_command(
    socket: &mut networking::Socket<shared::message::ClientMessage, shared::message::ServerMessage>,
    music_player: &mut Box<dyn crate::player::Player>,
    command: shared::command::PlayerCommand,
) -> crate::player::Result<()> {
    use shared::command::PlayerCommand;
    use shared::message::ServerMessage;

    match command {
        PlayerCommand::Play => {
            music_player.play()?;

            // This will error if the server is not connected to any client but we don't care as the server is autonomous
            let _ = socket.send(ServerMessage::PlayerStatePlay);
        }
        PlayerCommand::Pause => {
            music_player.pause()?;

            let _ = socket.send(ServerMessage::PlayerStatePause);
        }
        PlayerCommand::GetPlayState => {
            let state = match &music_player.is_playing() {
                true => ServerMessage::PlayerStatePlay,
                false => ServerMessage::PlayerStatePause,
            };

            let _ = socket.send(state);
        }
        PlayerCommand::GetCurrentlyPlaying => {
            let song = music_player.currently_playing()?;
            let index = music_player.currently_playing_index()?;

            let _ = socket.send(ServerMessage::CurrentlyPlaying { song, index });
        }
        PlayerCommand::GetQueue => {
            let _ = socket.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }
        PlayerCommand::AddToQueue(id) => {
            music_player.add_queue(id)?;

            let _ = socket.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }
        PlayerCommand::RemoveFromQueue(i) => {
            music_player.remove_queue(i)?;

            let _ = socket.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }
        PlayerCommand::ClearQueue => {
            music_player.clear_queue()?;

            let _ = socket.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }

        PlayerCommand::SetVolume(val) => {
            music_player.set_volume(val.max(0.))?;
            let _ = socket.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }
        PlayerCommand::GetVolume => {
            let _ = socket.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }

        PlayerCommand::GetDeviceName => {
            let _ = socket.send(ServerMessage::AudioDevice(music_player.audio_device()?));
        }
        PlayerCommand::SetDeviceByName(new_device_name) => {
            use shared::error::server::{Error, PlayerError};

            let message = match music_player.set_device_by_name(&new_device_name) {
                Ok(_) => ServerMessage::AudioDevice(new_device_name),
                Err(e) => ServerMessage::Error(Error::Player(PlayerError::SetDeviceError {
                    device: new_device_name,
                    e: e.to_string(),
                })),
            };
            let _ = socket.send(message);
        }

        PlayerCommand::SetPosition(pos) => {
            music_player.set_pos(pos)?;
            let _ = socket.send(ServerMessage::Position(music_player.pos()?)); // qol for socket.sync
        }
        PlayerCommand::GetPosition => {
            let _ = socket.send(ServerMessage::Position(music_player.pos()?));
        }
    }
    Ok(())
}

pub fn handle_downloader_command(
    socket: &mut networking::Socket<shared::message::ClientMessage, shared::message::ServerMessage>,
    download_mgr: &mut crate::downloader::DownloadManager,
    command: shared::command::DownloaderCommand,
) {
    use shared::{
        command::DownloaderCommand, error::server::DownloaderError, message::ServerMessage,
    };

    match command {
        DownloaderCommand::QueueDownload(url) => {
            const YOUTUBE_VIDEO_ID_LEN: usize = 11;

            if !url.starts_with("https://youtube.com/")
                && !url.starts_with("https://www.youtube.com/")
                && !url.starts_with("https://music.youtube.com/")
                && !{ url.len() == YOUTUBE_VIDEO_ID_LEN && !url.contains(['.', ':', '/']) }
            {
                if let Err(e) = socket.send(ServerMessage::Error(
                    DownloaderError::UrlParse {
                        url,
                        reason: String::from(
                            "Only youtube, youtube music links or youtube video ids are allowed.",
                        ),
                    }
                    .into(),
                )) {
                    error!("Failed to send back PlaylistNotSupported error to client due to: {e}");
                };
                return;
            }

            if url.contains("/playlist?") || url.contains("&list=") {
                if let Err(e) = socket.send(ServerMessage::Error(
                    DownloaderError::PlaylistNotSupported.into(),
                )) {
                    error!("Failed to send back PlaylistNotSupported error to client due to: {e}");
                };
                return;
            }

            // debug!("Received dl request");
            // let config = crate::downloader::DownloadConfig { url };

            // let mut handle = crate::downloader::download(config);

            // // let's do blocking for now
            // let mut latest_prcentage = 0.;
            // while handle.running() {
            //     handle.update();
            //     if handle.download_percentage() != latest_prcentage {
            //         latest_prcentage = handle.download_percentage();
            //         debug!("{latest_prcentage}%")
            //     }
            // }

            download_mgr.push(crate::downloader::DownloadConfig {
                url,
                song_path: crate::config::songs_path(),
            });
        }
        DownloaderCommand::StopCurrentDownload => unimplemented!(),
        DownloaderCommand::FetchCurrent => {
            let reports = download_mgr.report_currents();
            if let Err(e) = socket.send(ServerMessage::CurrentDownloads(reports)) {
                error!("Failed to send download reports to client due to: {e}");
            }
        }
        DownloaderCommand::FetchQueue => unimplemented!(),
    }
}

fn handle_playlist_command(
    socket: &mut networking::Socket<shared::message::ClientMessage, shared::message::ServerMessage>,
    pc: shared::command::PlaylistCommand,
) {
    use shared::message::ServerMessage;

    match pc {
        shared::command::PlaylistCommand::GetAll => {}
        shared::command::PlaylistCommand::GetOne(uuid) => {
            match shared::playlist::Playlist::load(uuid, &crate::config::songs_path()) {
                Ok(playlist) => {
                    if let Err(e) = socket.send(ServerMessage::Playlist(playlist)) {
                        error!("Failed to respond to playlist({uuid}) request due to: {e}");
                    }
                }
                Err(e) => {
                    if let Err(e) =
                        socket.send(ServerMessage::Error(crate::error::Error::Playlist(e)))
                    {
                        error!("Failed to send playlist({uuid}) load error({e}) due to: {e}")
                    }
                }
            }
        }
        shared::command::PlaylistCommand::Create(playlist) => todo!(),
        shared::command::PlaylistCommand::Delete(uuid) => {
            match shared::playlist::Playlist::delete(uuid, &crate::config::songs_path()) {
                Ok(_) => {
                    if let Err(e) = socket.send(ServerMessage::PlaylistDeleted(uuid)) {
                        error!("Failed to send playlist({uuid}) deletion response due to: {e}");
                    }
                }
                Err(e) => {
                    if let Err(e) =
                        socket.send(ServerMessage::Error(crate::error::Error::Playlist(e)))
                    {
                        error!("Failed to send playlist({uuid}) deletion error due to: {e}");
                    }
                }
            }
        }
        shared::command::PlaylistCommand::Rename(uuid, new_name) => {
            let mut playlist =
                match shared::playlist::Playlist::load(uuid, &crate::config::songs_path()) {
                    Ok(p) => p,
                    Err(e) => panic!("{e}"),
                };

            playlist.set_name(new_name);

            match playlist.save(&crate::config::songs_path()) {
                Ok(_) => {
                    if let Err(e) = socket.send(ServerMessage::PlaylistUpdated(playlist)) {
                        error!(
                            "Failed to send playlist({uuid}) update message to client due to: {e}"
                        )
                    }
                }
                Err(e) => {
                    if let Err(e) =
                        socket.send(ServerMessage::Error(crate::error::Error::Playlist(e)))
                    {
                        error!("Failed to send playlist({uuid}) update error to client due to: {e}")
                    }
                }
            }
        }
        shared::command::PlaylistCommand::AddToPlaylist {
            playlist_uuid,
            song_uuid,
        } => {
            let mut playlist =
                match shared::playlist::Playlist::load(playlist_uuid, &crate::config::songs_path())
                {
                    Ok(p) => p,
                    Err(e) => panic!("{e}"),
                };

            // FIXME: Rework Errors to allow for more abstract errors like 'Failed to read song file' instead of failed to read file /a/b/c.meta
            let song = shared::song::Song::load(song_uuid, &crate::config::songs_path()).unwrap();

            playlist.songs_mut().push(song);

            match playlist.save(&crate::config::songs_path()) {
                Ok(_) => {
                    let uuid = *playlist.uuid();
                    if let Err(e) = socket.send(ServerMessage::PlaylistUpdated(playlist)) {
                        error!(
                            "Failed to send playlist({uuid}) update message to client due to: {e}"
                        )
                    }
                }
                Err(e) => {
                    let uuid = *playlist.uuid();
                    if let Err(e) =
                        socket.send(ServerMessage::Error(crate::error::Error::Playlist(e)))
                    {
                        error!("Failed to send playlist({uuid}) update error to client due to: {e}")
                    }
                }
            }
        }
        shared::command::PlaylistCommand::RemoveFromPlaylist {
            playlist_uuid,
            song_index,
        } => {
            let mut playlist =
                match shared::playlist::Playlist::load(playlist_uuid, &crate::config::songs_path())
                {
                    Ok(p) => p,
                    Err(e) => panic!("{e}"),
                };

            // FIXME: This can panic if the index is outside the bounds
            playlist.songs_mut().remove(song_index as usize);

            match playlist.save(&crate::config::songs_path()) {
                Ok(_) => {
                    let uuid = *playlist.uuid();
                    if let Err(e) = socket.send(ServerMessage::PlaylistUpdated(playlist)) {
                        error!(
                            "Failed to send playlist({uuid}) update message to client due to: {e}"
                        )
                    }
                }
                Err(e) => {
                    let uuid = *playlist.uuid();
                    if let Err(e) =
                        socket.send(ServerMessage::Error(crate::error::Error::Playlist(e)))
                    {
                        error!("Failed to send playlist({uuid}) update error to client due to: {e}")
                    }
                }
            }
        }
    }
}
