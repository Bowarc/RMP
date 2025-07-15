use shared::{
    command::{Command, PlayerCommand},
    message::{ClientMessage, ServerMessage},
};

pub fn get(client: &mut shared::client::Client) -> f32 {
    crate::send_and_wait!(client, Command::Player(
            PlayerCommand::GetVolume,
        ), f32, ServerMessage::PlayerVolume(vol) => vol)
}

pub fn set(client: &mut shared::client::Client, amnt: f32) {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::SetVolume(amnt),
        )))
        .unwrap();

    debug!("{:?}", client.recv(std::time::Duration::from_secs(1)))
}

pub fn up(client: &mut shared::client::Client, amnt: f32) {
    let current = get(client);

    set(client, current + amnt)
}

pub fn down(client: &mut shared::client::Client, amnt: f32) {
    let current = get(client);

    set(client, current - amnt)
}
