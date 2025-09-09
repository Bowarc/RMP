pub fn render(
    ui: &mut egui::Ui,
    ectx: &egui::Context,
    client: &mut client::Client,
    music_player_last_update_request: &mut std::time::Instant,
    polling_rate: &std::time::Duration,
) {
    use std::time::Duration;
    let mut to_send = Vec::new();

    if client.player_data().playing {
        // ectx.request_repaint();

        if music_player_last_update_request
            .elapsed()
            .as_secs_f32()
            > polling_rate.as_secs_f32() * 1.1
        {
            *music_player_last_update_request = std::time::Instant::now();
            to_send.push(shared::message::ClientMessage::Command(
                shared::command::PlayerCommand::GetPosition.into(),
            ));
            to_send.push(shared::message::ClientMessage::Command(
                shared::command::PlayerCommand::GetPlayState.into(),
            ));
        }
    }

    ui.horizontal(|ui| {
        // Create the sidebar for songs
        ui.vertical(|ui| {
            ui.set_min_width(150.0);
            ui.label("Songs");
            for song in client.player_data().song_list.iter() {
                if ui.button(song.metadata().title()).clicked() {
                    if client.player_data().playing {
                        to_send.push(shared::message::ClientMessage::Command(
                            shared::command::PlayerCommand::ClearQueue.into(),
                        ));
                    }

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
                }
            }
        });

        // Main content area
        ui.vertical(|ui| {
            ui.label("Now Playing:");
            if let Some(song) = &client.player_data().current_song {
                ui.label(song.metadata().title());
                ui.horizontal(|ui| {
                    ui.label("Position:");
                    let mut p = client.player_data().position.as_secs_f32();
                    let resp = ui.add(
                        egui::Slider::new(&mut p, 0.0..=song.metadata().duration().as_secs_f32())
                            .show_value(false),
                    );
                    ui.label(format!("{p:.2} seconds"));
                    if resp.changed() && client.player_data().playing {
                        to_send.push(shared::message::ClientMessage::Command(
                            shared::command::PlayerCommand::SetPosition(Duration::from_secs_f32(p))
                                .into(),
                        ));
                    }
                });
                ui.horizontal(|ui| {
                    static mut POS: Option<f32> = None;
                    const MIN_OUTPUT: f32 = 0.0;
                    const MAX_OUTPUT: f32 = 3.0;

                    #[allow(static_mut_refs)]
                    if unsafe { POS.is_none() } {
                        unsafe {
                            POS = Some(
                                ((client.player_data().volume - MIN_OUTPUT)
                                    / (MAX_OUTPUT - MIN_OUTPUT))
                                    .sqrt(),
                            );
                        }
                    }

                    #[allow(static_mut_refs)]
                    let pos = unsafe { POS.as_mut().unwrap() };

                    ui.label("Volume:");
                    let resp = ui.add(
                        egui::Slider::new(pos, 0.0f32..=1f32)
                            .step_by(0.001)
                            .show_value(false),
                    );
                    let output = MIN_OUTPUT + (MAX_OUTPUT - MIN_OUTPUT) * pos.powf(2.0);
                    ui.label(format!("{output:.2}"));

                    if resp.changed() {
                        to_send.push(shared::message::ClientMessage::Command(
                            shared::command::PlayerCommand::SetVolume(output).into(),
                        ));
                    }
                });

                if ui
                    .button(if client.player_data().playing {
                        "Pause"
                    } else {
                        "Play"
                    })
                    .clicked()
                {
                    let cmd = match client.player_data().playing {
                        true => shared::command::PlayerCommand::Pause,
                        false => shared::command::PlayerCommand::Play,
                    };
                    to_send.push(shared::message::ClientMessage::Command(cmd.into()));
                    ectx.request_repaint();
                    ectx.request_repaint_after(std::time::Duration::from_secs_f32(1.));
                }
            } else {
                ui.label("Select a song from the sidebar.");
            }
        });
    });

    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a player message due to: {e}");
    }
}
