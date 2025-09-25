pub fn render(ui: &mut egui::Ui, mut max_rect: egui::Rect, client: &mut client::Client) {
    let mut to_send = Vec::new();

    static mut ANIMATION_OFFSET: f32 = 0.;

    let id = egui::Id::new("song_queue_animation_t");

    let get =
        |ui: &egui::Ui| -> Option<std::time::Instant> { ui.memory(|mem| mem.data.get_temp(id)) };

    let set =
        |ui: &egui::Ui, val: std::time::Instant| ui.memory_mut(|mem| mem.data.insert_temp(id, val));
    let clear = |ui: &egui::Ui| ui.memory_mut(|mem| mem.data.remove::<std::time::Instant>(id));

    if ui.button("Activate").clicked() {
        error!("SET");
        set(ui, std::time::Instant::now());
        unsafe { ANIMATION_OFFSET = 0. }
    }

    if let Some(_t) = get(ui) {
        if unsafe { ANIMATION_OFFSET < max_rect.width() * 0.33 } {
            // TODO: use unstable_dt from egui context
            println!("woo");
            unsafe { ANIMATION_OFFSET += 3. }
        }
    } else {
        return;
    }

    max_rect.min.x = max_rect.max.x - unsafe { ANIMATION_OFFSET };
    ui.scope_builder(
        egui::UiBuilder::new()
            .max_rect(max_rect)
            .layout(egui::Layout::right_to_left(egui::Align::TOP)),
        |ui| {
            let frame = egui::Frame::new()
                .fill(ui.visuals_mut().window_fill)
                .corner_radius(egui::CornerRadius {
                    nw: 10,
                    ne: 0,
                    sw: 10,
                    se: 0,
                })
                .inner_margin(egui::Margin {
                    left: 10,
                    top: 5,
                    bottom: 10,
                    ..Default::default()
                })
                .outer_margin(egui::Margin {
                    left: 10,
                    right: 0,
                    ..Default::default()
                })
                .stroke(egui::Stroke::new(1., egui::Color32::WHITE));

            frame.show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(egui::RichText::new("Queue").heading().underline());
                    ui.add_space(10.);
                    egui::ScrollArea::vertical()
                        .max_height(400.)
                        .show(ui, |ui| {
                            for song in client.player_data().song_queue.iter() {
                                if ui.button(song.metadata().title()).clicked() {
                                    debug!("Queue song clicked: {}", song.metadata().title());
                                }
                            }
                        })
                });
            })
        },
    );


    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a player message due to: {e}");
    }
}
