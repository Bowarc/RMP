pub fn render(ui: &mut egui::Ui, client: &mut client::Client) {
    // Quality code right there LMFAOO
    static mut ALL_SONGS: bool = false;

    let mut to_send = Vec::new();

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

                let mut force_open = false;
                egui::ComboBox::from_id_salt("tst")
                    .close_behavior(if force_open {
                        egui::PopupCloseBehavior::IgnoreClicks
                    } else {
                        egui::PopupCloseBehavior::CloseOnClickOutside
                    })
                    // .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
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
                            //
                            // .. : In a selectable value, if the 'current_value'(arg0) == 'selected_value(arg1)
                            //     egui displays it as 'selected', but since I kinda do my own selection system with
                            //     'virtual' playlist ALL_SONGS, using it normally would show a playlist highlighted
                            //     even when ALL_SONG is on.
                            let resp = if unsafe { ALL_SONGS } {
                                // And () == ()
                                //    =   vv   =
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
                            if if playlist.uuid() == &current_uuid && unsafe { !ALL_SONGS } {
                                ui.selectable_value(&mut (), (), playlist.name())
                            } else {
                                ui.selectable_value(&mut 0, 1, playlist.name())
                            }
                            .clicked()
                            {
                                client.playlist_data_mut().current = Some(*playlist.uuid());
                                unsafe { ALL_SONGS = false };
                            }
                        }
                        ui.add_space(10.);

                        // TODO: CLEAN THIS WHOLE CODE BLOCK
                        super::center(
                            ui,
                            |ui| {
                                // if ui.add_sized(egui::Vec2::new(5., 1.), egui::Button::new("+")).clicked(){
                                //     info!("Hi")
                                // }
                                let resp = ui.button("+");
                                static mut TAG: bool = false;
                                if resp.clicked() {
                                    debug!("Triggered");
                                    unsafe { TAG = !TAG };
                                }
                                if unsafe { TAG } {
                                    let resp = egui::containers::Popup::menu(&resp)
                                        .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
                                        .align(egui::RectAlign::from_align2(
                                            egui::Align2::CENTER_CENTER,
                                        ))
                                        .open(true)
                                        .show(
                                            #[allow(static_mut_refs)] // Safety: trust me
                                            // Tbf, this is only ever gonna be single threaded
                                            // so using mutable statics is fine
                                            |ui| 't: {
                                                static mut TEXT: String = String::new();
                                                let teresp =
                                                    ui.text_edit_singleline(unsafe { &mut TEXT });

                                                teresp.request_focus();
                                                // if teresp.has_focus() {
                                                //     force_open = true;
                                                // }

                                                if ui.button("Create !").clicked() {
                                                    if unsafe { TEXT.is_empty() } {
                                                        break 't;
                                                    }
                                                    unsafe {
                                                        let t = &TEXT;
                                                        info!("Create new playlist with name: {t}");
                                                    }
                                                    let new_playlist =
                                                        shared::playlist::Playlist::new(unsafe {
                                                            TEXT.clone()
                                                        });

                                                    to_send.extend_from_slice(&[
                                                        shared::message::ClientMessage::Command(shared::command::PlaylistCommand::Create(new_playlist).into()),
                                                        shared::message::ClientMessage::Command(shared::command::PlaylistCommand::GetAll.into()),
                                                    ]);

                                                    unsafe {
                                                        TEXT.clear();
                                                    }
                                                }
                                            },
                                        );
                                    if resp.unwrap().response.should_close() && !force_open {
                                        unsafe { TAG = false }
                                    }
                                }
                            },
                            "New playlist center button",
                        );
                    });

                if client.playlist_data().current.is_none() && unsafe { !ALL_SONGS } {
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

    let mut request_play = |song: &shared::song::Song| {
        to_send.push(shared::message::ClientMessage::Command(
            shared::command::PlayerCommand::AddToQueue(song.uuid()).into(),
        ));

        // This is to be able to add a song to a paused existing queue and keep the player paused
        if !client.player_data().playing && client.player_data().song_queue.is_empty() {
            to_send.push(shared::message::ClientMessage::Command(
                shared::command::PlayerCommand::Play.into(),
            ));
        }

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
