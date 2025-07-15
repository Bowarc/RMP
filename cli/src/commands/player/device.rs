use shared::{
    command::{Command, PlayerCommand},
    message::{ClientMessage, ServerMessage},
};

pub fn get(client: &mut shared::client::Client) -> String {
    crate::send_and_wait!(client, Command::Player(PlayerCommand::GetDeviceName),String, ServerMessage::AudioDevice(name) => name)
}

pub fn set(client: &mut shared::client::Client, device_name: String) {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::SetDeviceByName(device_name),
        )))
        .unwrap();

    debug!("{:?}", client.recv(std::time::Duration::from_secs(1)))
}
