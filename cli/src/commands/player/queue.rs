use shared::{
    command::{Command, PlayerCommand},
    message::{ClientMessage, ServerMessage},
};

pub enum SongIdentifier {
    Uuid(uuid::Uuid),
    Title(String),
}

pub fn get(client: &mut shared::client::Client) -> Vec<shared::song::Song> {
    client
        .send(ClientMessage::Command(Command::Player(
            PlayerCommand::GetQueue,
        )))
        .unwrap();

    loop {
        let Ok((_, message)) = client.recv(std::time::Duration::from_secs(1)) else {
            panic!("Huh")
        };

        match message {
            ServerMessage::PlayerQueue(vol) => return vol,
            ServerMessage::Error(e) => {
                panic!("{e}")
            }
            _ => unreachable!(),


        }
    }
}

pub fn add(client: &mut shared::client::Client, si: SongIdentifier) {
    match si {
        SongIdentifier::Uuid(uuid) => {
            client
                .send(ClientMessage::Command(Command::Player(
                    PlayerCommand::AddToQueue(uuid),
                )))
                .unwrap();
            debug!("{:?}", client.recv(std::time::Duration::from_secs(1)));
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
