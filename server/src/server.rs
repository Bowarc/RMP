// Not a big fan of oop but let's try and we'll see later it that causes a problem
// Wasn't long lmao

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
        }
    };

    let Some(socket) = socket_opt else {
        return;
    };

    loop {
        let (header, cm) = match socket.try_recv() {
            Ok(message) => message,
            Err(e) => {
                warn!("Client disconnected");
                *socket_opt = None;
                break;
            }
        };

        info!("Received a message from client");

        match cm {
            ClientMessage::Command(command) => match command {
                Command::Player(pc) => {
                    if let Err(e) = handle_player_command(socket, music_player, pc) {
                        let _ =
                            socket.send(ServerMessage::Error(Error::Player(e)));
                    }
                }
                Command::Downloader(dc) => {
                    handle_downloader_command(socket, download_mgr, dc)
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
}

pub fn handle_player_command(
    socket: &mut networking::Socket<
        shared::message::ClientMessage,
        shared::message::ServerMessage,
    >,
    music_player: &mut Box<dyn crate::player::Player>,
    command: shared::command::PlayerCommand,
) -> crate::player::Result<()> {
    use shared::command::PlayerCommand;
    use shared::message::ServerMessage;

    match command {
        PlayerCommand::Play => {
            music_player.play()?;

            // This will error if the server is not connected to any client, but we don't care as the server is autonomous */
            let _ = socket.send(ServerMessage::PlayerStatePlay);
        }
        PlayerCommand::Pause => {
            music_player.pause()?;

            let _ = socket.send(ServerMessage::PlayerStatePause);
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
        PlayerCommand::RemoveFromQueue(id) => {
            music_player.remove_queue(id)?;

            let _ = socket.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }
        PlayerCommand::ClearQueue => {
            music_player.clear_queue()?;

            let _ = socket.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }

        PlayerCommand::SetVolume(val) => {
            music_player.set_volume(val)?;
            let _ = socket.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }
        PlayerCommand::GetVolume => {
            let _ = socket.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }

        PlayerCommand::GetDeviceName => {
            let _ = socket.send(ServerMessage::AudioDevice(music_player.audio_device()?));
        }
        PlayerCommand::SetDeviceByName(new_device_name) => {
            use shared::server::error::{Error, PlayerError};

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
            let _ = socket.send(ServerMessage::Position(music_player.pos()?)); // qol for client sync
        }
        PlayerCommand::GetPosition => {
            let _ = socket.send(ServerMessage::Position(music_player.pos()?));
        }
    }
    Ok(())
}

pub fn handle_downloader_command(
    socket: &mut networking::Socket<
        shared::message::ClientMessage,
        shared::message::ServerMessage,
    >,
    download_mgr: &mut crate::downloader::DownloadManager,
    command: shared::command::DownloaderCommand,
) {
    use shared::command::DownloaderCommand;

    match command {
        DownloaderCommand::QueueDownload(url) => {
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

            download_mgr.push(crate::downloader::DownloadConfig { url });
        }
        DownloaderCommand::StopCurrentDownload => unimplemented!(),
        DownloaderCommand::FetchCurrent => unimplemented!(),
        DownloaderCommand::FetchQueue => unimplemented!(),
    }
}
