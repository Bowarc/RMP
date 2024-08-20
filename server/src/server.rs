// Not a big fan of oop but let's try and we'll see later it that causes a problem
// Wasn't long lmao

use std::os::windows::io::NullHandleError;

use networking::{proxy, Message};

pub fn recv_commands(
    proxy_opt: &mut Option<
        networking::proxy::ProxyController<
            shared::message::ClientMessage,
            shared::message::ServerMessage,
        >,
    >,
    music_player: &mut Box<dyn crate::player::Player>,
) {
    use shared::message::{ClientMessage, ServerMessage};

    let Some(proxy) = proxy_opt else {
        return;
    };

    loop {
        let pm = match proxy.try_recv() {
            Ok(message) => message,
            Err(e) => {
                if e != std::sync::mpsc::TryRecvError::Empty{
                    error!("{e}");
                    *proxy_opt = None;
                }
                break;
            }
        };
        let cm = match pm {
            proxy::ProxyMessage::Forward(msg) => msg,
            proxy::ProxyMessage::ConnectionResetError | proxy::ProxyMessage::Exit => {
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
                    error!(
                        "Failed to send back pong message to client due to: {e}",
                    );
                }
            }
            ClientMessage::Pong => (),
        }
    }
}

pub fn handle_player_command(
    proxy: &mut networking::proxy::ProxyController<shared::message::ClientMessage, shared::message::ServerMessage>,
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
        PlayerCommand::AddToQueue(id) => {
            music_player.add_queue(id)?;

            let _ = proxy.send(ServerMessage::PlayerQueueUpdated(music_player.queue()?));
        }
        PlayerCommand::RemoveFromQueue(id) => {
            music_player.remove_queue(id)?;

            let _ = proxy.send(ServerMessage::PlayerQueueUpdated(music_player.queue()?));
        }
        PlayerCommand::ClearQueue => {
            music_player.clear_queue()?;

            let _ = proxy.send(ServerMessage::PlayerQueueUpdated(music_player.queue()?));
        }
        PlayerCommand::SetVolume(val) => {
            music_player.set_volume(val)?;
            let _ = proxy.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }
        PlayerCommand::GetVolume => {
            let _ = proxy.send(ServerMessage::PlayerVolume(music_player.volume()?));
        }
    }
    Ok(())
}

pub fn handle_downloader_command(
    proxy: &mut networking::proxy::ProxyController<shared::message::ClientMessage, shared::message::ServerMessage>,
    music_player: &mut Box<dyn crate::player::Player>,
    command: shared::command::DownloaderCommand,
) {
    use shared::command::DownloaderCommand;

    match command {
        DownloaderCommand::StartDownload(url) => unimplemented!(),
        DownloaderCommand::StopCurrentDownload => unimplemented!(),
        DownloaderCommand::FetchCurrent => unimplemented!(),
        DownloaderCommand::FetchQueue => unimplemented!(),
    }
}
