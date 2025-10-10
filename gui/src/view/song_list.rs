pub fn render(ui: &mut egui::Ui, client: &mut client::Client) {
    // Quality code right there LMFAOO
    static mut ALL_SONGS: bool = false;

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

    if !client.playlist_data().all.is_empty() {
        super::center(
            ui,
            |ui| {
                #[allow(clippy::useless_vec)]
                // let playlists = &client.playlist_data().all;
                let current_playlist = client
                    .playlist_data()
                    .current
                    .and_then(|uuid| client.playlist_data().get(&uuid))
                    .unwrap_or_else(|| client.playlist_data().all.first().unwrap());

                let current_uuid = *current_playlist.uuid();

                egui::ComboBox::from_id_salt("tst")
                    .selected_text(
                        egui::RichText::new(if unsafe { ALL_SONGS } {
                            "Library"
                        } else {
                            current_playlist.name()
                        })
                        .heading()
                        .size(23.),
                    )
                    .show_ui(ui, |ui| {
                        {
                            // Quality code right there LMFAOO
                            let resp = if unsafe { ALL_SONGS } {
                                ui.selectable_value(&mut (), (), "All")
                            } else {
                                ui.selectable_value(&mut 0, 1, "All")
                            };
                            if resp.clicked() {
                                client.playlist_data_mut().current = None;
                                unsafe { ALL_SONGS = true };
                            }
                        }
                        for i in 0..client.playlist_data().all.len() {
                            let playlist = client.playlist_data().all.get(i).unwrap();

                            // Quality code right there LMFAOO
                            let resp = if playlist.uuid() == &current_uuid && unsafe { !ALL_SONGS }
                            {
                                ui.selectable_value(&mut 0, 0, playlist.name())
                            } else {
                                ui.selectable_value(&mut 0, 1, playlist.name())
                            };

                            if resp.clicked() {
                                info!("Changed to: {}", playlist.uuid());
                                client.playlist_data_mut().current = Some(*playlist.uuid());
                                unsafe { ALL_SONGS = false };
                            }
                        }
                    });

                if client.playlist_data().current.is_none() && unsafe { !ALL_SONGS } {
                    debug!("Current set");
                    client.playlist_data_mut().current =
                        Some(*client.playlist_data().all.first().unwrap().uuid());
                }
            },
            "song_list_playlist_name",
        );

        ui.add_space(30.);
    }

    let song_list = if unsafe { ALL_SONGS } || client.playlist_data().current.is_none() {
        &client.player_data().song_list
    } else {
        client.playlist_data().get_current().unwrap().songs()
    };

    // let song_list = match client.playlist_data().get_current() {
    //     Some(p) => p.songs(),
    //     None => &client.player_data().song_list,
    // };

    super::center(
        ui,
        |ui| {
            ui.vertical(|ui| {
                ui.set_min_width(150.0);

                for song in song_list.iter() {
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
