use shared::{
    command::{Command, PlayerCommand},
    message::ClientMessage,
};

pub enum SongIdentifier {
    Uuid(uuid::Uuid),
    Title(String),
}

pub fn add(client: &mut shared::client::Client, si: SongIdentifier) {
    match si {
        SongIdentifier::Uuid(uuid) => {
            client
                .send(ClientMessage::Command(Command::Player(
                    PlayerCommand::AddToQueue(uuid),
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

pub fn remove(client: &mut shared::client::Client, si: SongIdentifier) {
    match si {
        SongIdentifier::Uuid(uuid) => {
            client
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

pub fn clear(client: &mut shared::client::Client) {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::ClearQueue,
        )))
        .unwrap();
}
