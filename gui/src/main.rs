#[macro_use]
extern crate log;

use eframe::egui;

mod view;

enum Tab {
    MusicPlayer,
    Downloads,
}

struct Interface<'c> {
    client: &'c mut client::Client,
    current_tab: Tab,

    music_player_last_update_request: std::time::Instant,
    downloader_last_update_request: std::time::Instant,
    polling_rate: std::time::Duration,
}

impl<'c> Interface<'c> {
    fn new(client: &'c mut client::Client, _cc: &eframe::CreationContext) -> Self {
        let ping = *client.ping();

        Self {
            client,
            current_tab: Tab::MusicPlayer,

            music_player_last_update_request: std::time::Instant::now(),
            downloader_last_update_request: std::time::Instant::now(),
            polling_rate: ping,
        }
    }
}

impl<'c> Interface<'c> {
    fn request_info_update(&mut self) {
        if self.client.player_data().playing {
            // ectx.request_repaint();

            if self
                .music_player_last_update_request
                .elapsed()
                .as_secs_f32()
                > self.polling_rate.as_secs_f32() * 1.1
            {
                self.music_player_last_update_request = std::time::Instant::now();
                // TODO: error mngment
                let _ = self.client.send_multiple(vec![
                    shared::message::ClientMessage::Command(
                        shared::command::PlayerCommand::GetPosition.into(),
                    ),
                    shared::message::ClientMessage::Command(
                        shared::command::PlayerCommand::GetPlayState.into(),
                    ),
                    shared::message::ClientMessage::Command(
                        shared::command::PlayerCommand::GetCurrentlyPlaying.into(),
                    ),
                ]);
            }
        }

        if self.downloader_last_update_request.elapsed().as_secs_f32()
            > self.polling_rate.as_secs_f32() * 1.1
        {
            self.downloader_last_update_request = std::time::Instant::now();

            // TODO: error mngment
            let _ = self
                .client
                .send_multiple(vec![shared::message::ClientMessage::Command(
                    shared::command::DownloaderCommand::FetchCurrent.into(),
                )]);
        }
    }
}

impl<'c> eframe::App for Interface<'c> {
    fn update(&mut self, ectx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the current position if playing

        match self.client.update() {
            Ok(_message_count) => (),
            Err(e) => {
                error!("Failed to update due to: {e}");

                *self.client = create_client();
            }
        }
        ectx.request_repaint();
        ectx.set_debug_on_hover(true);

        self.request_info_update();

        egui::CentralPanel::default()
            .frame(
                eframe::egui::Frame::NONE
                    .fill(ectx.style().visuals.window_fill())
                    .corner_radius(10.0)
                    .stroke(ectx.style().visuals.widgets.noninteractive.fg_stroke)
                    .outer_margin(0.5),
            )
            .show(ectx, |ui| {
                const TITLE_BAR_HEIGHT: f32 = 32.0;

                let app_rect = ui.max_rect();

                // draw the title bar
                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + TITLE_BAR_HEIGHT;
                    rect
                };

                view::title_bar::render(ui, ectx, title_bar_rect);

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
                                if let Err(e) = self.client.socket_mut().send(
                                    shared::message::ClientMessage::Command(
                                        shared::command::Command::GetLibrary,
                                    ),
                                ) {
                                    error!("Failed to request for the library due to: {e}")
                                };
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

                let general_content =
                    ui.scope_builder(egui::UiBuilder::new().max_rect(unallocated_size), |ui| {
                        ui.style_mut().spacing.indent = 10.;
                        match self.current_tab {
                            Tab::MusicPlayer => {
                                view::player_tab::render(ui, ectx, self.client);
                            }
                            Tab::Downloads => {
                                view::downloader_tab::downloader_tab(ui, ectx, self.client);
                            }
                        }
                    });

                let currently_playing_bar_size = {
                    const TAB_SELECT_PADDING: f32 = 10.;
                    let mut rect = unallocated_size;
                    rect.min.y = unallocated_size.min.y
                        + (general_content.response.rect.max.y
                            - general_content.response.rect.min.y)
                        + TAB_SELECT_PADDING;
                    rect
                }
                .shrink(4.0);

                view::currently_playing_bar::render(ui, ectx, currently_playing_bar_size, self.client);
            });
    }
}

fn create_client() -> client::Client {
    use shared::{
        command::{Command, PlayerCommand},
        message::ClientMessage,
    };

    let mut client = client::Client::init();

    if let Err(e) = client.send_multiple(vec![
        ClientMessage::Command(PlayerCommand::GetVolume.into()),
        ClientMessage::Command(PlayerCommand::GetCurrentlyPlaying.into()),
        ClientMessage::Command(PlayerCommand::GetPlayState.into()),
        ClientMessage::Command(Command::GetLibrary),
    ]) {
        error!("Failed to send an init message due to: {e}");
    }

    client
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
    let mut client = create_client();

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
