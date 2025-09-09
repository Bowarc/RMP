pub fn render(
    ui: &mut egui::Ui,
    ectx: &egui::Context,
    available_rect: eframe::epaint::Rect,
) {
    const TITLE: &str = "RMP GUI";

    let painter = ui.painter();

    let title_bar_response = ui.interact(
        available_rect,
        egui::Id::new("title_bar"),
        egui::Sense::click(),
    );

    // Paint the title:
    painter.text(
        available_rect.center(),
        eframe::emath::Align2::CENTER_CENTER,
        TITLE,
        eframe::epaint::FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // Paint the line under the title:
    painter.line_segment(
        [
            available_rect.left_bottom() + eframe::epaint::vec2(1.0, 0.0),
            available_rect.right_bottom() + eframe::epaint::vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    // Interact with the title bar (drag to move window):
    if title_bar_response.double_clicked() {
        // frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        ectx.send_viewport_cmd(egui::viewport::ViewportCommand::StartDrag);
        // frame.drag_window();
    }

    // Show toggle button for light/dark mode
    ui.scope_builder(egui::UiBuilder::new().max_rect(available_rect), |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            egui::global_theme_preference_switch(ui);
            // egui::widgets::global_dark_light_mode_switch(ui);
        });
    });

    // Show some close/maximize/minimize buttons for the native window.
    ui.scope_builder(egui::UiBuilder::new().max_rect(available_rect), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);

            let button_height = 12.0;

            if ui
                .add(egui::Button::new(
                    egui::RichText::new("❌").size(button_height),
                ))
                .on_hover_text("Close the window")
                .clicked()
            {
                ectx.send_viewport_cmd(egui::viewport::ViewportCommand::Close);
            }

            let (hover_text, clicked_state) = if ui.input(|i| i.viewport().maximized) == Some(true)
            {
                ("Restore window", false)
            } else {
                ("Maximize window", true)
            };

            if ui
                .add(egui::Button::new(
                    egui::RichText::new("🗗").size(button_height),
                ))
                .on_hover_text(hover_text)
                .clicked()
            {
                if clicked_state {
                    ectx.send_viewport_cmd(egui::viewport::ViewportCommand::Maximized(true));
                } else {
                    ectx.send_viewport_cmd(egui::viewport::ViewportCommand::Maximized(false));
                }
            }

            if ui
                .add(egui::Button::new(
                    egui::RichText::new("🗕").size(button_height),
                ))
                .on_hover_text("Minimize the window")
                .clicked()
            {
                ectx.send_viewport_cmd(egui::viewport::ViewportCommand::Minimized(true));
            }
        });
    });
}
