// Not a big fan of oop but let's try and we'll see later it that causes a problem
// Wasn't long lmao

pub fn recv_commands(
    proxy_opt: &mut Option<
        networking::proxy::ProxyController<
            shared::message::ClientMessage,
            shared::message::ServerMessage,
        >,
    >,
    music_player: &mut Box<dyn crate::player::Player>,
) {
    use {
        networking::proxy::ProxyMessage,
        shared::message::{ClientMessage, ServerMessage},
    };

    let Some(proxy) = proxy_opt else {
        return;
    };

    loop {
        let pm = match proxy.try_recv() {
            Ok(message) => message,
            Err(e) => {
                if e != std::sync::mpsc::TryRecvError::Empty {
                    error!("{e}");
                    *proxy_opt = None;
                }
                break;
            }
        };
        let cm = match pm {
            ProxyMessage::Forward(msg) => msg,
            ProxyMessage::ConnectionResetError | ProxyMessage::Exit => {
                *proxy_opt = None;
                break;
            }
        };

        info!("Received a message from client");

        match cm {
            ClientMessage::Command(command) => match command {
                shared::command::Command::Player(pc) => {
                    if let Err(e) = handle_player_command(proxy, music_player, pc) {
                        let _ =
                            proxy.send(ServerMessage::Error(crate::error::Error::PlayerError(e)));
                    }
                }
                shared::command::Command::Downloader(dc) => {
                    handle_downloader_command(proxy, music_player, dc)
                }
            },
            ClientMessage::Ping => {
                if let Err(e) = proxy.send(ServerMessage::Pong) {
                    error!("Failed to send back pong message to client due to: {e}",);
                }
            }
            ClientMessage::Pong => (),
        }
    }
}

pub fn handle_player_command(
    proxy: &mut networking::proxy::ProxyController<
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
            let _ = proxy.send(ServerMessage::PlayerStatePlay);
        }
        PlayerCommand::Pause => {
            music_player.pause()?;

            let _ = proxy.send(ServerMessage::PlayerStatePause);
        }
        PlayerCommand::GetCurrentlyPlaying => {
            let song = music_player.currently_playing()?;
            let index = music_player.currently_playing_index()?;

            let _ = proxy.send(ServerMessage::CurrentlyPlaying { song, index });
        }
        PlayerCommand::GetQueue => {
            let _ = proxy.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }
        PlayerCommand::AddToQueue(id) => {
            music_player.add_queue(id)?;

            let _ = proxy.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }
        PlayerCommand::RemoveFromQueue(id) => {
            music_player.remove_queue(id)?;

            let _ = proxy.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }
        PlayerCommand::ClearQueue => {
            music_player.clear_queue()?;

            let _ = proxy.send(ServerMessage::PlayerQueue(music_player.queue()?));
        }

        PlayerCommand::SetVolume(val) => {
            music_player.set_volume(val)?;
            let _ = proxy.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }
        PlayerCommand::GetVolume => {
            let _ = proxy.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }

        PlayerCommand::GetDeviceName => {
            let _ = proxy.send(ServerMessage::AudioDevice(music_player.audio_device()?));
        }
        PlayerCommand::SetDeviceByName(new_device_name) => {
            use shared::server::error::{Error, PlayerError};

            let message = match music_player.set_device_by_name(&new_device_name) {
                Ok(_) => ServerMessage::AudioDevice(new_device_name),
                Err(e) => ServerMessage::Error(Error::PlayerError(PlayerError::SetDeviceError {
                    device: new_device_name,
                    e: e.to_string(),
                })),
            };
            let _ = proxy.send(message);
        }

        PlayerCommand::SetPosition(pos) => {
            music_player.set_pos(pos)?;
            let _ = proxy.send(ServerMessage::Position(music_player.pos()?)); // qol for client sync
        }
        PlayerCommand::GetPosition => {
            let _ = proxy.send(ServerMessage::Position(music_player.pos()?));
        }
    }
    Ok(())
}

pub fn handle_downloader_command(
    _proxy: &mut networking::proxy::ProxyController<
        shared::message::ClientMessage,
        shared::message::ServerMessage,
    >,
    _music_player: &mut Box<dyn crate::player::Player>,
    command: shared::command::DownloaderCommand,
) {
    use shared::command::DownloaderCommand;

    match command {
        DownloaderCommand::StartDownload(url) => {
            debug!("Received dl request");
            let config = crate::downloader::DownloadConfig { url };

            let mut handle = crate::downloader::download(config);

            // let's do blocking for now
            let mut latest_prcentage = 0.;
            while handle.running() {
                handle.update();
                if handle.download_percentage() != latest_prcentage {
                    latest_prcentage = handle.download_percentage();
                    debug!("{latest_prcentage}%")
                }
            }
        }
        DownloaderCommand::StopCurrentDownload => unimplemented!(),
        DownloaderCommand::FetchCurrent => unimplemented!(),
        DownloaderCommand::FetchQueue => unimplemented!(),
    }
}
