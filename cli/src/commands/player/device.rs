use shared::{
    command::{Command, PlayerCommand},
    message::{ClientMessage, ServerMessage},
};

pub fn get(client: &mut shared::client::Client) -> String {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::GetDeviceName,
        )))
        .unwrap();

    loop {
        let Ok((_, message)) = client.recv(std::time::Duration::from_secs(1)) else {
            panic!("Huh")
        };

        match message {
            ServerMessage::AudioDevice(name) => return name,
            ServerMessage::Error(e) => {
                panic!("{e}")
            }
            _ => unreachable!(),
        }
    }
}

pub fn set(client: &mut shared::client::Client, device_name: String) {
    client.send(ClientMessage::Command(Command::Player(
        PlayerCommand::SetDeviceByName(device_name),
    ))).unwrap();

    debug!("{:?}", client.recv(std::time::Duration::from_secs(1)))
}
