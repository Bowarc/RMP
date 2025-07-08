pub mod device;
pub mod position;
pub mod queue;
pub mod volume;

type Client = shared::client::Client;

use shared::command::{Command, PlayerCommand};
use shared::message::{ClientMessage, ServerMessage};
use std::time::Duration;

pub fn play(client: &mut Client) {
    if let Err(e) = client.send(ClientMessage::Command(Command::Player(PlayerCommand::Play))) {
        error!("{e}");
    }

    debug!("{:?}", client.recv(Duration::from_secs(1)))
}

pub fn pause(client: &mut Client) {
    if let Err(e) = client.send(ClientMessage::Command(Command::Player(
        PlayerCommand::Pause,
    ))) {
        error!("{e}");
    }

    debug!("{:?}", client.recv(Duration::from_secs(1)))
}

pub fn now_playing(client: &mut Client) -> (shared::song::Song, u64) {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::GetCurrentlyPlaying,
        )))
        .unwrap();

    let message = match client.recv(std::time::Duration::from_secs(1)) {
        Ok((_header, message)) => message,
        Err(e) => {
            panic!("{e}");
        }
    };

    match message {
        ServerMessage::CurrentlyPlaying { song, index } => (song, index),
        ServerMessage::Error(e) => {
            panic!("{e}")
        }
        _ => unreachable!(),
    }
}
