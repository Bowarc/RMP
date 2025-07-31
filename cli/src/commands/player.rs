pub mod device;
pub mod position;
pub mod queue;
pub mod volume;

type Client = shared::Socket;

use shared::command::{Command, PlayerCommand};
use shared::message::{ClientMessage, ServerMessage};
use std::time::Duration;

pub fn play(socket: &mut Client) {
    if let Err(e) = socket.send(ClientMessage::Command(Command::Player(PlayerCommand::Play))) {
        error!("{e}");
    }

    debug!("{:?}", socket.recv(Duration::from_secs(1)))
}

pub fn pause(socket: &mut Client) {
    if let Err(e) = socket.send(ClientMessage::Command(Command::Player(
        PlayerCommand::Pause,
    ))) {
        error!("{e}");
    }

    debug!("{:?}", socket.recv(Duration::from_secs(1)))
}

pub fn now_playing(socket: &mut Client) -> (shared::song::Song, u64) {
    crate::send_and_wait!(socket, Command::Player(PlayerCommand::GetCurrentlyPlaying),String, ServerMessage::CurrentlyPlaying{song, index} => (song, index))
}
