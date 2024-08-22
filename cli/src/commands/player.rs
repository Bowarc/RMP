pub mod queue;
pub mod volume;
pub mod device;

type Client = shared::client::Client;

pub fn play(client: &mut Client) {
    use {
        shared::{
            command::{Command, PlayerCommand},
            message::{ClientMessage, ServerMessage},
        },
        std::time::Duration,
    };

    if let Err(e) = client.send(ClientMessage::Command(shared::command::Command::Player(
        shared::command::PlayerCommand::Play,
    ))) {
        error!("{e}");
    }

    debug!("{:?}", client.recv(Duration::from_secs(1)))
}

pub fn pause(client: &mut Client) {
    use {
        shared::{
            command::{Command, PlayerCommand},
            message::{ClientMessage, ServerMessage},
        },
        std::time::Duration,
    };

    if let Err(e) = client.send(ClientMessage::Command(shared::command::Command::Player(
        shared::command::PlayerCommand::Pause,
    ))) {
        error!("{e}");
    }

    debug!("{:?}", client.recv(Duration::from_secs(1)))
}
