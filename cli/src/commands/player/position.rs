use {
    shared::{
        command::{Command, PlayerCommand},
        message::{ClientMessage, ServerMessage},
    },
    std::time::Duration,
};

pub fn get(client: &mut shared::client::Client) -> Duration {
    crate::send_and_wait!(client, Command::Player(PlayerCommand::GetPosition), std::time::Duration, ServerMessage::Position(duration) => duration)
}

pub fn set(client: &mut shared::client::Client, value: Duration) {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::SetPosition(value),
        )))
        .unwrap();

    debug!("{:?}", client.recv(Duration::from_secs(1)))
}

pub fn forward(client: &mut shared::client::Client, amnt: Duration) {
    let current = get(client);

    set(client, current + amnt)
}

pub fn backward(client: &mut shared::client::Client, amnt: Duration) {
    let current = get(client);

    set(client, current - amnt)
}
