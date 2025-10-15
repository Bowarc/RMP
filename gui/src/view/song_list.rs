pub fn render(ui: &mut egui::Ui, client: &mut client::Client) {
    // Quality code right there LMFAOO
    static mut ALL_SONGS: bool = false;
    static mut EDIT_BUTTONS: bool = false;

    let mut to_send = Vec::new();
    
    if !client.playlist_data().all.is_empty() {
        #[allow(static_mut_refs)]
        playlist_selection(ui, client, unsafe{&mut ALL_SONGS}, &mut to_send);
    }else{
        super::center(ui,
            #[allow(static_mut_refs)]
             |ui|{
                 let res = ui.button("New playlist");
                 create_playlist_pop_up(&res, &mut to_send, false);
            
        }, "song list lone create playlist button");
    }
    
    super::center(ui, |ui|{
        if !(unsafe { ALL_SONGS } || client.playlist_data().current.is_none())
            && ui.button("Edit").clicked()
        {
            unsafe { EDIT_BUTTONS = !EDIT_BUTTONS }
        }
    }, "song list edit buttons");

    debug!("{:?}", client.playlist_data().get_current());
    let song_list = if unsafe { ALL_SONGS } || client.playlist_data().current.is_none() {
        unsafe { EDIT_BUTTONS = true };
        &client.player_data().song_list
    } else {
        client.playlist_data().get_current().unwrap().songs()
    };

    let request_play = |song: &shared::song::Song,
                        to_send: &mut Vec<shared::message::ClientMessage>| {
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
                    ui.horizontal(|ui| {

                            
                        if unsafe {EDIT_BUTTONS}{
                            ui.horizontal_wrapped(|ui| {
                                egui::containers::Popup::menu(&ui.button("a"))
                                    .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
                                    .align(egui::RectAlign::from_align2(egui::Align2::CENTER_CENTER))
                                    // .open(true)
                                    .show(|ui| {
                                        for playlist in &client.playlist_data().all {
                                            if ui.button(playlist.name()).clicked() {
                                                to_send.extend_from_slice(&[
                                                    shared::message::ClientMessage::Command(shared::command::PlaylistCommand::AddToPlaylist { playlist_uuid: *playlist.uuid(), song_uuid: song.uuid() }.into()),
                                                    shared::message::ClientMessage::Command(shared::command::PlaylistCommand::GetAll.into())
                                                ]);
                                            }
                                        }
                                    });
                            });

                            if !(unsafe { ALL_SONGS } || client.playlist_data().current.is_none())
                                && ui.button("x").clicked(){
                                let playlist =client.playlist_data().get_current().unwrap();
                                to_send.extend_from_slice(&[
                                    shared::message::ClientMessage::Command(
                                        shared::command::PlaylistCommand::RemoveFromPlaylist {
                                            playlist_uuid: *playlist.uuid(),
                                            song_index: playlist.songs()
                                                .iter()
                                                .position(|s| s.uuid() == song.uuid())
                                                .unwrap() as u16
                                        }.into()),
                                        shared::message::ClientMessage::Command(
                                            shared::command::PlaylistCommand::GetAll.into()
                                        )
                                ]);
                            }
                        }
                        // ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(song.metadata().title()).clicked() {
                            request_play(song, &mut to_send)
                        }
                        // })
                    });
                }
            });
        },
        "song_list",
    );

    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a player message due to: {e}");
    }
}



fn playlist_selection(ui: &mut egui::Ui, client: &mut client::Client, all_songs: &mut bool, to_send: &mut Vec<shared::message::ClientMessage>){
    super::center(ui,|ui| {
        // let playlists = &client.playlist_data().all;
        let current_playlist = client
            .playlist_data()
            .current
            .and_then(|uuid| client.playlist_data().get(&uuid))
            .unwrap_or_else(|| client.playlist_data().all.first().unwrap());

        let current_uuid = *current_playlist.uuid();

        // This id is to make the combo box height auto reset when we add a new playlist (or remove one)
        // Else, the height does not change and we have to scroll to see all playlists and the + button
        egui::ComboBox::from_id_salt(client.playlist_data().all.len())
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .selected_text(
                egui::RichText::new(if *all_songs  {
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
                    let resp = if *all_songs  {
                        // And () == ()
                        //    =   vv   =
                        ui.selectable_value(&mut (), (), "All")
                    } else {
                        ui.selectable_value(&mut 0, 1, "All")
                    };
                    if resp.clicked() {
                        client.playlist_data_mut().current = None;
                        *all_songs = true;
                        ui.close();
                    }
                }

                for i in 0..client.playlist_data().all.len() {
                    ui.horizontal(|ui|{

                        // Is it a good idea to be able to delete a whole playlist that easily ?
                        // 
                        // let puuid = *client.playlist_data().all.get(i).unwrap().uuid();

                        // if ui.button("X").clicked(){
                        //     error!("Delete playlist: {}", puuid);
                        //     if Some(&puuid) == client.playlist_data().get_current().map(|p|p.uuid()){
                        //         *all_songs = true;
                        //         client.playlist_data_mut().current = None;
                        //     }
                        //     to_send.extend_from_slice(&[
                        //         shared::message::ClientMessage::Command(
                        //             shared::command::PlaylistCommand::Delete(puuid).into()
                        //         ),
                        //         shared::message::ClientMessage::Command(
                        //             shared::command::PlaylistCommand::GetAll.into()
                        //         )
                        //     ]);
                        // }
                
                        let playlist = client.playlist_data().all.get(i).unwrap();

                        // Quality code right there LMFAOO
                        if if playlist.uuid() == &current_uuid && !*all_songs  {
                            ui.selectable_value(&mut (), (), playlist.name())
                        } else {
                            ui.selectable_value(&mut 0, 1, playlist.name())
                        }
                        .clicked()
                        {
                            client.playlist_data_mut().current = Some(*playlist.uuid());
                            *all_songs = false;
                            ui.close();
                        }
                    });
                }

                ui.add_space(10.);

                // TODO: CLEAN THIS WHOLE CODE BLOCK
                super::center(ui, |ui| {
                    #[allow(static_mut_refs)]
                    static mut FORCE_OPEN: bool = false;
                    let resp = ui.button("+");
                    if resp.clicked(){
                        unsafe{FORCE_OPEN = true}
                    }
                    
                    let res = create_playlist_pop_up(&resp, to_send, unsafe{FORCE_OPEN});
                    
                    if unsafe{FORCE_OPEN} && res.unwrap().response.should_close(){
                        unsafe{FORCE_OPEN = false}
                    }
                }, "New playlist center button");
            });

        if client.playlist_data().current.is_none() && !*all_songs  {
            client.playlist_data_mut().current =
                Some(*client.playlist_data().all.first().unwrap().uuid());
        }
    }, "song_list_playlist_name");
    ui.add_space(30.);
}

fn create_playlist_pop_up(button_resp: &egui::Response, to_send: &mut Vec<shared::message::ClientMessage>, force_open: bool) -> Option<egui::InnerResponse<()>> {
    debug!("Hi :3");
    let mut menu  =egui::containers::Popup::from_toggle_button_response(button_resp)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .align(egui::RectAlign::from_align2(
            egui::Align2::CENTER_CENTER,
        ));
        // .kind(egui::PopupKind::Menu)
        // .open(true)

    if force_open{
        menu = menu.open(true)
    }
    
    menu.show(
            #[allow(static_mut_refs)] // Safety: trust me
            // Tbf, this is only ever gonna be single threaded
            // so using mutable statics is fine
            |ui| {
                error!("hehe");
                static mut TEXT: String = String::new();
                let text = unsafe{ &mut TEXT };
                let teresp = ui.text_edit_singleline(text);
                teresp.request_focus();
                if text.is_empty() || !(ui.button("Create !").clicked() || teresp.ctx.input(|i| i.key_pressed(egui::Key::Enter))) {
                    return;
                }
                let new_playlist =
                    shared::playlist::Playlist::new(text.clone());
                to_send.extend_from_slice(&[
                    shared::message::ClientMessage::Command(shared::command::PlaylistCommand::Create(new_playlist).into()),
                    shared::message::ClientMessage::Command(shared::command::PlaylistCommand::GetAll.into()),
                ]);
                ui.close();
                text.clear();
            },
        )
}
