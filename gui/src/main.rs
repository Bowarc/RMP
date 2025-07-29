use eframe::egui;

const TITLE_BAR_HEIGHT: f32 = 32.0;

struct Download {
    url: String,
    progress: f32,
}

enum Tab {
    MusicPlayer,
    Downloads,
}
struct MyApp {
    songs: Vec<String>,
    selected_song: Option<usize>,
    is_playing: bool,
    current_position_s: f32,
    elapsed_time_s: f32,
    downloads: Vec<Download>,
    new_download_url: String,
    current_tab: Tab,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            songs: vec![
                "Song 1".to_string(),
                "Song 2".to_string(),
                "Song 3".to_string(),
                "Song 4".to_string(),
            ],
            selected_song: None,
            is_playing: false,
            current_position_s: 0.0,
            elapsed_time_s: 0.0,
            downloads: Vec::new(),
            new_download_url: String::new(),
            current_tab: Tab::MusicPlayer,
        }
    }
}

impl MyApp {
    fn render_title_bar(
        &mut self,
        ui: &mut egui::Ui,
        ectx: &egui::Context,
        title_bar_rect: eframe::epaint::Rect,
        title: &str,
    ) {
        let painter = ui.painter();

        let title_bar_response = ui.interact(
            title_bar_rect,
            egui::Id::new("title_bar"),
            egui::Sense::click(),
        );

        // Paint the title:
        painter.text(
            title_bar_rect.center(),
            eframe::emath::Align2::CENTER_CENTER,
            title,
            eframe::epaint::FontId::proportional(20.0),
            ui.style().visuals.text_color(),
        );

        // Paint the line under the title:
        painter.line_segment(
            [
                title_bar_rect.left_bottom() + eframe::epaint::vec2(1.0, 0.0),
                title_bar_rect.right_bottom() + eframe::epaint::vec2(-1.0, 0.0),
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
        ui.scope_builder(egui::UiBuilder::new().max_rect(title_bar_rect), |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.visuals_mut().button_frame = false;
                ui.add_space(8.0);
                egui::global_theme_preference_switch(ui);
                // egui::widgets::global_dark_light_mode_switch(ui);
            });
        });

        // Show some close/maximize/minimize buttons for the native window.
        ui.scope_builder(egui::UiBuilder::new().max_rect(title_bar_rect), |ui| {
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

                let (hover_text, clicked_state) =
                    if ui.input(|i| i.viewport().maximized) == Some(true) {
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

    fn player_tab(&mut self, ui: &mut egui::Ui, ectx: &egui::Context) {
        if self.is_playing {
            ectx.request_repaint();
            self.elapsed_time_s += 0.1; // Simulate time passing

            if self.elapsed_time_s >= 0.1 {
                self.current_position_s += 0.1; // Increment by 0.1 seconds
                self.elapsed_time_s = 0.0; // Reset the timer

                if self.current_position_s > 100.0 {
                    self.current_position_s = 100.0; // Stop at 100 seconds
                    self.is_playing = false; // Stop playing when the song ends
                }
            }
        }
        ui.horizontal(|ui| {
            // Create the sidebar for songs
            ui.vertical(|ui| {
                ui.set_min_width(150.0);
                ui.label("Songs");
                for (i, song) in self.songs.iter().enumerate() {
                    if ui.button(song).clicked() {
                        self.selected_song = Some(i);
                        self.current_position_s = 0.0; // Reset position when a new song is selected
                        self.is_playing = true; // Start playing the new song
                    }
                }
            });

            // Main content area
            ui.vertical(|ui| {
                ui.label("Now Playing:");
                if let Some(index) = self.selected_song {
                    ui.label(&self.songs[index]);
                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        ui.add(
                            egui::Slider::new(&mut self.current_position_s, 0.0..=100.0)
                                .text("seconds"),
                        );
                    });

                    if ui
                        .button(if self.is_playing { "Pause" } else { "Play" })
                        .clicked()
                    {
                        self.is_playing = !self.is_playing;
                    }
                } else {
                    ui.label("Select a song from the sidebar.");
                }
            });
        });
    }
    fn downloader_tab(&mut self, ui: &mut egui::Ui, ectx: &egui::Context) {
        ui.group(|ui| {
            ui.label("Download Music");
            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.text_edit_singleline(&mut self.new_download_url);
                if ui.button("Download").clicked() {
                    // Simulate adding a download
                    self.downloads.push(Download {
                        url: self.new_download_url.clone(),
                        progress: 0.0,
                    });
                    self.new_download_url.clear(); // Clear the input field
                }
            });
            // Display download queue
            for download in &mut self.downloads {
                ui.horizontal(|ui| {
                    ui.label(&download.url);
                    let progress = &mut download.progress;
                    ui.horizontal(|ui| {
                        ui.set_min_width(12. * 3.);
                        ui.label(format!("{:.0}%", *progress * 100.0));
                    });
                    ui.add(egui::ProgressBar::new(*progress).desired_width(100.0));
                    // Simulate progress for demonstration
                    *progress += 0.003; // Increment progress
                    if *progress >= 1.0 {
                        *progress = 1.0; // Cap at 100%
                    }
                });
            }
            if self.downloads.iter().any(|d| d.progress != 1.) {
                ectx.request_repaint();
            }

            self.downloads.retain(|d| d.progress != 1.);
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ectx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the current position if playing

        egui::CentralPanel::default()
            .frame(
                eframe::egui::Frame::NONE
                    .fill(ectx.style().visuals.window_fill())
                    .corner_radius(10.0)
                    .stroke(ectx.style().visuals.widgets.noninteractive.fg_stroke)
                    .outer_margin(0.5),
            )
            .show(ectx, |ui| {
                let app_rect = ui.max_rect();

                // draw the title bar

                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + TITLE_BAR_HEIGHT;
                    rect
                };
                self.render_title_bar(ui, ectx, title_bar_rect, "RMP Gui");

                let unallocated_size = {
                    let mut rect = app_rect;
                    rect.min.y = title_bar_rect.max.y;
                    rect.max.y = app_rect.max.y;
                    rect
                }
                .shrink(4.0);

                let tab_bar =
                    ui.scope_builder(egui::UiBuilder::new().max_rect(unallocated_size), |ui| {
                        ui.horizontal(|ui| {
                            if ui.button("Music Player").clicked() {
                                self.current_tab = Tab::MusicPlayer;
                            }
                            if ui.button("Downloads").clicked() {
                                self.current_tab = Tab::Downloads;
                            }
                        });
                    });

                let unallocated_size = {
                    let mut rect = unallocated_size;
                    rect.min.y = unallocated_size.min.y
                        + (tab_bar.response.rect.max.y - tab_bar.response.rect.min.y);
                    rect
                }
                .shrink(4.0);

                ui.scope_builder(egui::UiBuilder::new().max_rect(unallocated_size), |ui| {
                    ui.style_mut().spacing.indent = 10.;
                    match self.current_tab {
                        Tab::MusicPlayer => {
                            self.player_tab(ui, ectx);
                        }
                        Tab::Downloads => {
                            self.downloader_tab(ui, ectx);
                        }
                    }
                })
            });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(eframe::egui::vec2(800.0, 600.0))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_title("RMP Gui"),
        ..Default::default()
    };
    eframe::run_native(
        "RMP Gui",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
