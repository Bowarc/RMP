use shared::{
    command::{Command, PlayerCommand},
    message::{ClientMessage, ServerMessage},
};

pub fn get(socket: &mut shared::Socket) -> f32 {
    crate::send_and_wait!(socket, Command::Player(
            PlayerCommand::GetVolume,
        ), f32, ServerMessage::PlayerVolume(vol) => vol)
}

pub fn set(socket: &mut shared::Socket, amnt: f32) {
    socket
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::SetVolume(amnt),
        )))
        .unwrap();

    debug!("{:?}", socket.recv(std::time::Duration::from_secs(1)))
}

pub fn up(socket: &mut shared::Socket, amnt: f32) {
    let current = get(socket);

    set(socket, current + amnt)
}

pub fn down(socket: &mut shared::Socket, amnt: f32) {
    let current = get(socket);

    set(socket, current - amnt)
}
