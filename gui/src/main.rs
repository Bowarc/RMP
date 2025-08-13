#[macro_use]
extern crate log;

use std::{f32, str::FromStr};

use eframe::egui;

const TITLE_BAR_HEIGHT: f32 = 32.0;

// struct Download {
//     url: String,
//     progress: f32,
// }

enum Tab {
    MusicPlayer,
    Downloads,
}
struct Interface<'c> {
    client: &'c mut client::App,
    current_tab: Tab,

    music_player_last_udpate_request: std::time::Instant,
    polling_rate: std::time::Duration,
}

impl<'c> Interface<'c> {
    fn new(client: &'c mut client::App, _cc: &eframe::CreationContext) -> Self {
        if let Err(e) = client.send_multiple(vec![
            shared::message::ClientMessage::Command(
                shared::command::PlayerCommand::GetVolume.into(),
            ),
            // shared::message::ClientMessage::Command(
            //     shared::command::PlayerCommand::AddToQueue(
            //         uuid::Uuid::from_str("4c40abd6-f2ed-47f8-9e6e-b235a8980835").unwrap(),
            //     )
            //     .into(),
            // ),
            // shared::message::ClientMessage::Command(shared::command::PlayerCommand::Play.into()),
            shared::message::ClientMessage::Command(
                shared::command::PlayerCommand::GetCurrentlyPlaying.into(),
            ),
            shared::message::ClientMessage::Command(
                shared::command::PlayerCommand::GetPlayState.into(),
            ),
            shared::message::ClientMessage::Command(shared::command::Command::GetLibrary),
        ]) {
            error!("Failed to send an init message due to: {e}");
        }

        let ping = client.ping().clone();
        Self {
            client,
            current_tab: Tab::MusicPlayer,
            music_player_last_udpate_request: std::time::Instant::now(),
            polling_rate: ping,
        }
    }
}

impl<'c> Interface<'c> {
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
        use std::time::Duration;
        let mut to_send = Vec::new();

        if self.client.player_data().playing {
            // ectx.request_repaint();

            if self
                .music_player_last_udpate_request
                .elapsed()
                .as_secs_f32()
                > self.polling_rate.as_secs_f32() * 1.1
            {
                self.music_player_last_udpate_request = std::time::Instant::now();
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
                for song in self.client.player_data().song_list.iter() {
                    if ui.button(song.metadata().title()).clicked() {
                        if self.client.player_data().playing {
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
                if let Some(song) = &self.client.player_data().current_song {
                    ui.label(song.metadata().title());
                    ui.horizontal(|ui| {
                        ui.label("Position:");
                        let mut p = self.client.player_data().position.as_secs_f32();
                        let resp = ui.add(
                            egui::Slider::new(
                                &mut p,
                                0.0..=song.metadata().duration().as_secs_f32(),
                            )
                            .text("seconds"),
                        );
                        if resp.changed()

                        // p != self.client.player_data().position.as_secs_f32()

                             && self.client.player_data().playing
                        {
                            to_send.push(shared::message::ClientMessage::Command(
                                shared::command::PlayerCommand::SetPosition(
                                    Duration::from_secs_f32(p),
                                )
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
                                    ((self.client.player_data().volume - MIN_OUTPUT)
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
                                shared::command::PlayerCommand::SetVolume(
                                    output
                                )
                                .into(),
                            ));
                        }
                    });

                    if ui
                        .button(if self.client.player_data().playing {
                            "Pause"
                        } else {
                            "Play"
                        })
                        .clicked()
                    {
                        let cmd = match self.client.player_data().playing {
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

        if let Err(e) = self.client.send_multiple(to_send) {
            error!("Failed to send a player message due to: {e}");
        }
    }
    fn downloader_tab(&mut self, ui: &mut egui::Ui, ectx: &egui::Context) {
        let mut to_send = Vec::new();
        ui.group(|ui| {
            ui.label("Download Music");
            ui.horizontal(|ui| {
                ui.label("URL:");
                ui.text_edit_singleline(&mut self.client.downloader_data_mut().new_download_url);
                if ui.button("Download").clicked() {
                    // Simulate adding a download
                    to_send.push(shared::message::ClientMessage::Command(
                        shared::command::DownloaderCommand::QueueDownload(
                            self.client.downloader_data().new_download_url.clone(),
                        )
                        .into(),
                    ));
                    self.client.downloader_data_mut().new_download_url.clear(); // Clear the input field
                }
            });

            // Display download queue
            for download in self
                .client
                .downloader_data_mut()
                .current_downloads
                .iter_mut()
            {
                // TODO: Implement a way to display currently running downloads
                // ui.horizontal(|ui| {
                //     ui.label(&download.url);
                //     let progress = &mut download.progress;
                //     ui.horizontal(|ui| {
                //         ui.set_min_width(12. * 3.);
                //         ui.label(format!("{:.0}%", *progress * 100.0));
                //     });
                //     ui.add(egui::ProgressBar::new(*progress).desired_width(100.0));
                //     // Simulate progress for demonstration
                //     *progress += 0.003; // Increment progress
                //     if *progress >= 1.0 {
                //         *progress = 1.0; // Cap at 100%
                //     }
                // });
            }
        });

        if let Err(e) = self.client.send_multiple(to_send) {
            error!("Failed to send a downloader message due to: {e}");
        }
    }
}

impl<'c> eframe::App for Interface<'c> {
    fn update(&mut self, ectx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the current position if playing

        self.client.update();
        // if self.client.update() > 0 {
        // std::thread::sleep_ms(500);
        ectx.request_repaint();
        // }

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

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let mut p = self.polling_rate.as_millis() as u64;

                                let resp = ui.add(
                                    egui::DragValue::new(&mut p)
                                        .range(30..=500)
                                        .speed(5)
                                        .prefix("Polling rate: "),
                                );
                                if resp.changed() {
                                    self.polling_rate = std::time::Duration::from_millis(p);
                                }
                            })
                        });
                    });

                let unallocated_size = {
                    const TAB_SELECT_PADDING: f32 = 10.;
                    let mut rect = unallocated_size;
                    rect.min.y = unallocated_size.min.y
                        + (tab_bar.response.rect.max.y - tab_bar.response.rect.min.y)
                        + TAB_SELECT_PADDING;
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
    logger::init(
        logger::Config::default()
            .output(logger::Output::Stdout)
            .filters(&[
                ("networking", log::LevelFilter::Warn),
                ("eframe", log::LevelFilter::Error),
                ("egui_glow", log::LevelFilter::Error),
                ("calloop", log::LevelFilter::Error),
                ("egui_winit", log::LevelFilter::Error),
                ("smithay_client_toolkit", log::LevelFilter::Error),
                ("sctk_adwaita", log::LevelFilter::Off),
                ("arboard", log::LevelFilter::Off),
            ])
            .colored(true),
    );
    let mut client = client::App::init();

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size(eframe::egui::vec2(800.0, 600.0))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(true)
            .with_title("RMP Gui"),
        ..Default::default()
    };

    let gui_res = eframe::run_native(
        "RMP Gui",
        options,
        Box::new(|cc| Ok(Box::new(Interface::new(&mut client, cc)))),
    );

    client.exit();

    gui_res
}
