pub mod currently_playing_bar;
pub mod downloader_tab;
pub mod player_tab;
pub mod song_list;
pub mod title_bar;
pub mod song_queue;

pub fn center(
    ui: &mut eframe::egui::Ui,
    add_contents: impl FnOnce(&mut eframe::egui::Ui),
    identifier: impl std::hash::Hash,
) {
    ui.horizontal(|ui| {
        let id = ui.id().with(identifier);
        let last_width: Option<f32> = ui.memory_mut(|mem| mem.data.get_temp(id));
        if let Some(last_width) = last_width {
            ui.add_space((ui.available_width() - last_width) / 2.0);
        }
        let res = ui.scope(|ui| add_contents(ui)).response;
        let width = res.rect.width();
        ui.memory_mut(|mem| mem.data.insert_temp(id, width));

        // Repaint if width changed
        match last_width {
            None => ui.ctx().request_repaint(),
            Some(last_width) if last_width != width => ui.ctx().request_repaint(),
            Some(_) => {}
        }
    });
}
