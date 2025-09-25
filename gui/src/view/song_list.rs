pub fn render(ui: &mut egui::Ui, client: &mut client::Client) {
    let mut to_send = Vec::new();

    let mut request_play = |song: &shared::song::Song| {
        // to_send.push(shared::message::ClientMessage::Command(
        //     shared::command::PlayerCommand::ClearQueue.into(),
        // ));
        to_send.push(shared::message::ClientMessage::Command(
            shared::command::PlayerCommand::AddToQueue(song.uuid()).into(),
        ));

        to_send.push(shared::message::ClientMessage::Command(
            shared::command::PlayerCommand::Play.into(),
        ));
        to_send.push(shared::message::ClientMessage::Command(
            shared::command::PlayerCommand::GetCurrentlyPlaying.into(),
        ));
        to_send.push(shared::message::ClientMessage::Command(
            shared::command::PlayerCommand::GetPlayState.into(),
        ));
    };

    super::center(
        ui,
        |ui| {
            ui.vertical(|ui| {
                ui.set_min_width(150.0);

                for song in client.player_data().song_list.iter() {
                    if ui.button(song.metadata().title()).clicked() {
                        request_play(song)
                    }
                }
            });
        },
        "song_list",
    );

    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a player message due to: {e}");
    }
}

// super::center(
//     ui,
//     |ui| {
//         let id = egui::Id::new("current_playlist_name");
//         #[allow(clippy::useless_vec)]
//         let playlists = vec!["Playlist1", "Playlist2", "Playlist3"];

//         let get = || -> Option<String> { ui.memory(|mem| mem.data.get_temp(id)) };

//         let set = |ui: &egui::Ui, val: &str| ui.memory_mut(|mem| mem.data.insert_temp(id, val.to_string()));

//         if get().is_none() {
//             set(ui, playlists.first().unwrap())
//         }

//         let mut current_playlist_name = get().unwrap();

//         egui::ComboBox::from_id_salt("tst")
//             .selected_text(
//                 egui::RichText::new(&current_playlist_name)
//                     .heading()
//                     .size(23.),
//             )
//             .show_ui(ui, |ui| {
//                 for playlist in playlists.iter() {
//                     let resp = ui.selectable_value(
//                         &mut current_playlist_name,
//                         playlist.to_string(),
//                         *playlist,
//                     );

//                     if resp.changed() {
//                         println!("Changed to: {current_playlist_name}");
//                         set(ui, &current_playlist_name)
//                     }
//                 }
//             });
//     },
//     "song_list_playlist_name",
// );
