use {
    shared::{
        command::{Command, PlayerCommand},
        message::{ClientMessage, ServerMessage},
    },
    std::time::Duration,
};

pub fn get(socket: &mut shared::Socket) -> Duration {
    crate::send_and_wait!(socket, Command::Player(PlayerCommand::GetPosition), std::time::Duration, ServerMessage::Position(duration) => duration)
}

pub fn set(socket: &mut shared::Socket, value: Duration) {
    socket
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::SetPosition(value),
        )))
        .unwrap();

    debug!("{:?}", socket.recv(Duration::from_secs(1)))
}

pub fn forward(socket: &mut shared::Socket, amnt: Duration) {
    let current = get(socket);

    set(socket, current + amnt)
}

pub fn backward(socket: &mut shared::Socket, amnt: Duration) {
    let current = get(socket);

    set(socket, current - amnt)
}
