use shared::{
    command::{Command, PlayerCommand},
    message::{ClientMessage, ServerMessage},
};

pub fn get(socket: &mut shared::Socket) -> String {
    crate::send_and_wait!(socket, Command::Player(PlayerCommand::GetDeviceName),String, ServerMessage::AudioDevice(name) => name)
}

pub fn set(socket: &mut shared::Socket, device_name: String) {
    socket
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::SetDeviceByName(device_name),
        )))
        .unwrap();

    debug!("{:?}", socket.recv(std::time::Duration::from_secs(1)))
}
