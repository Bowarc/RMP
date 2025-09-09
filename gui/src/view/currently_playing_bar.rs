pub fn render(
    ui: &mut egui::Ui,
    ectx: &egui::Context,
    rect: egui::Rect,
    client: &mut client::Client,
) {
    use egui::UiBuilder;
    use std::time::Duration;

    let Some(song) = &client.player_data().current_song else {
        return;
    };

    let mut to_send = Vec::new();

    let ui_width = rect.max.x - rect.min.x;
    let ui_width_third = ui_width * 0.3;

    ui.scope_builder(
        UiBuilder::new()
            .max_rect({
                let mut r = rect;
                r.max.x = r.min.x + ui_width_third;
                r
            })
            .layout(egui::Layout::left_to_right(egui::Align::TOP)),
        |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            ui.label(song.metadata().title());
        },
    );

    ui.scope_builder(
        UiBuilder::new().max_rect({
            let mut r = rect;
            r.min.x += ui_width_third;
            r.max.x = r.min.x + ui_width_third;
            r
        }),
        |ui| {
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
                ectx.request_repaint_after(std::time::Duration::from_secs_f32(1.));
            }
            ui.horizontal(|ui| {
                ui.label("Position:");
                let mut p = client.player_data().position.as_secs_f32();
                let resp = ui.add(
                    egui::Slider::new(&mut p, 0.0..=song.metadata().duration().as_secs_f32())
                        .show_value(false)
                        .trailing_fill(true),
                );
                let pd = Duration::from_secs_f32(p);
                ui.label(format!("{} seconds", time::format(&pd, 2)));
                if resp.changed() {
                    to_send.push(shared::message::ClientMessage::Command(
                        shared::command::PlayerCommand::SetPosition(pd).into(),
                    ));
                }
            });
        },
    );

    ui.scope_builder(
        UiBuilder::new()
            .max_rect({
                let mut r = rect;
                r.min.x = r.max.x - ui_width_third;
                r
            })
            .layout(egui::Layout::left_to_right(egui::Align::TOP)),
        |ui| {
            static mut POS: Option<f32> = None;
            const MIN_OUTPUT: f32 = 0.0;
            const MAX_OUTPUT: f32 = 3.0;

            #[allow(static_mut_refs)]
            if unsafe { POS.is_none() } {
                unsafe {
                    POS = Some(
                        ((client.player_data().volume - MIN_OUTPUT) / (MAX_OUTPUT - MIN_OUTPUT))
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
                    .show_value(false)
                    .trailing_fill(true),
            );
            let output = MIN_OUTPUT + (MAX_OUTPUT - MIN_OUTPUT) * pos.powf(2.0);
            ui.label(format!("{output:.2}"));

            if resp.changed() {
                to_send.push(shared::message::ClientMessage::Command(
                    shared::command::PlayerCommand::SetVolume(output).into(),
                ));
            }
        },
    );

    // ui.horizontal(|ui| {
    //     // Left - Song title

    //     // Center - Pause button and position slider

    //     // Right - Volume slider
    //     ui.horizontal(|ui| {
    //     });
    // });

    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a player message due to: {e}");
    }
}
