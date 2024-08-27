use {
    shared::{
        command::{Command, PlayerCommand},
        message::{ClientMessage, ServerMessage},
    },
    std::time::Duration,
};

pub fn get(client: &mut shared::client::Client) -> Duration {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::GetPosition,
        )))
        .unwrap();

    loop {
        let Ok((_, message)) = client.recv(Duration::from_secs(1)) else {
            panic!("Huh")
        };

        match message {
            ServerMessage::Position(duration) => return duration,
            ServerMessage::Error(e) => {
                panic!("{e}")
            }
            _ => unreachable!(),
        }
    }
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
