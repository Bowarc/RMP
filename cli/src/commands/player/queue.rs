use shared::{
    command::{Command, PlayerCommand},
    message::{ClientMessage, ServerMessage},
};

pub enum SongIdentifier {
    Uuid(uuid::Uuid),
    Title(String),
}

pub fn get(socket: &mut shared::Socket) -> Vec<shared::song::Song> {
    crate::send_and_wait!(socket, Command::Player(PlayerCommand::GetQueue), Vec<shared::song::Song>, ServerMessage::PlayerQueue(queue) => queue)
}

pub fn add(socket: &mut shared::Socket, si: SongIdentifier) {
    match si {
        SongIdentifier::Uuid(uuid) => {
            socket
                .send(ClientMessage::Command(Command::Player(
                    PlayerCommand::AddToQueue(uuid),
                )))
                .unwrap();
            debug!("{:?}", socket.recv(std::time::Duration::from_secs(1)));
        }
        SongIdentifier::Title(_title) => {
            // Todo: Get the list of locally imported songs,
            // filter through them and file a match maybe using a fuzzy search

            unimplemented!()
        }
    }
}

pub fn remove(socket: &mut shared::Socket, si: SongIdentifier) {
    match si {
        SongIdentifier::Uuid(uuid) => {
            socket
                .send(ClientMessage::Command(Command::Player(
                    PlayerCommand::RemoveFromQueue(uuid),
                )))
                .unwrap();
        }
        SongIdentifier::Title(_title) => {
            // Todo: Get the list of locally imported songs,
            // filter through them and file a match maybe using a fuzzy search

            unimplemented!()
        }
    }
}

pub fn clear(socket: &mut shared::Socket) {
    socket
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::ClearQueue,
        )))
        .unwrap();
}
