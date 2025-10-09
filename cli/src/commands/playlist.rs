pub fn get_all(socket: &mut shared::Socket) -> Vec<shared::playlist::Playlist> {
    use shared::{
        command::{Command, PlaylistCommand},
        message::ServerMessage,
    };

    crate::send_and_wait!(socket, Command::Playlist(PlaylistCommand::GetAll), Vec<shared::playlist::Playlist>, ServerMessage::Playlists(playlists) => playlists)
}

pub fn get_one(socket: &mut shared::Socket, uuid: uuid::Uuid) -> shared::playlist::Playlist {
    use shared::{
        command::{Command, PlaylistCommand},
        message::ServerMessage,
    };

    crate::send_and_wait!(socket, Command::Playlist(PlaylistCommand::GetOne(uuid)), shared::playlist::Playlist, ServerMessage::Playlist(playlist) => playlist)
}

// TODO: Create

pub fn delete(socket: &mut shared::Socket, playlist_uuid: uuid::Uuid) {
    use shared::{command::PlaylistCommand, message::ClientMessage};

    socket
        .send(ClientMessage::Command(
            PlaylistCommand::Delete(playlist_uuid)
            .into(),
        ))
        .unwrap();
}

pub fn add_song(socket: &mut shared::Socket, playlist_uuid: uuid::Uuid, song_uuid: uuid::Uuid) {
    use shared::{command::PlaylistCommand, message::ClientMessage};

    socket
        .send(ClientMessage::Command(
            PlaylistCommand::AddToPlaylist {
                playlist_uuid,
                song_uuid,
            }
            .into(),
        ))
        .unwrap();
}

pub fn remove_song(socket: &mut shared::Socket, playlist_uuid: uuid::Uuid, song_index: u16) {
    use shared::{command::PlaylistCommand, message::ClientMessage};

    socket
        .send(ClientMessage::Command(
            PlaylistCommand::RemoveFromPlaylist {
                playlist_uuid,
                song_index,
            }
            .into(),
        ))
        .unwrap();
}

pub fn rename(socket: &mut shared::Socket, uuid: uuid::Uuid, new_name: String) {
    use shared::{command::PlaylistCommand, message::ClientMessage};

    let command = PlaylistCommand::Rename {
        playlist_uuid: uuid,
        new_name,
    }
    .into();
    socket.send(ClientMessage::Command(command)).unwrap();
}
