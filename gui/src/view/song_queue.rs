const TOTAL_ANIMATION_TIME_MS: f32 = 200.;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum AnimationSate {
    Open,
    Opening { start_time: std::time::Instant },
    Closed,
    Closing { start_time: std::time::Instant },
}

impl AnimationSate {
    pub fn closing() -> Self {
        Self::Closing {
            start_time: std::time::Instant::now(),
        }
    }
    pub fn opening() -> Self {
        Self::Opening {
            start_time: std::time::Instant::now(),
        }
    }
}

pub fn set(ui: &egui::Ui, state: AnimationSate) {
    ui.memory_mut(|mem| {
        mem.data
            .insert_temp("song_queue_animation_state".into(), state);
    })
}
pub fn get(ui: &egui::Ui) -> AnimationSate {
    ui.memory_mut(|mem| {
        mem.data
            .get_temp_mut_or_insert_with("song_queue_animation_state".into(), || {
                AnimationSate::Closed
            })
            .clone()
    })
}

pub fn flip(ui: &egui::Ui) {
    use std::time::{Duration, Instant};
    let current = get(ui);
    let new = match current {
        AnimationSate::Open => AnimationSate::closing(),
        AnimationSate::Closed => AnimationSate::opening(),
        AnimationSate::Opening { start_time } => {
            let elapsed = start_time.elapsed().as_millis() as f32;

            AnimationSate::Closing {
                start_time: Instant::now()
                    - Duration::from_millis((TOTAL_ANIMATION_TIME_MS - elapsed) as u64),
            }
        }
        AnimationSate::Closing { start_time } => {
            let elapsed = start_time.elapsed().as_millis() as f32;

            AnimationSate::Opening {
                start_time: Instant::now()
                    - Duration::from_millis((TOTAL_ANIMATION_TIME_MS - elapsed) as u64),
            }
        }
    };
    set(ui, new)
}

pub fn render(ui: &mut egui::Ui, mut max_rect: egui::Rect, client: &mut client::Client) {
    let mut to_send = Vec::new();

    let animation_state = get(ui);

    let range = ui.max_rect().width() * 0.33;
    let animation_offset = match animation_state {
        AnimationSate::Open => range,
        AnimationSate::Closed => return,
        AnimationSate::Opening { start_time } => {
            let ms = start_time.elapsed().as_millis() as f32;

            if ms > TOTAL_ANIMATION_TIME_MS {
                set(ui, AnimationSate::Open);
            }

            (ms / TOTAL_ANIMATION_TIME_MS).min(1.) * range
        }
        AnimationSate::Closing { start_time } => {
            let ms = start_time.elapsed().as_millis() as f32 + 1.;

            if ms > TOTAL_ANIMATION_TIME_MS {
                set(ui, AnimationSate::Closed);
            }

            let progress = (TOTAL_ANIMATION_TIME_MS - ms) / TOTAL_ANIMATION_TIME_MS;

            (progress * range).max(0.)
        }
    };

    max_rect.min.x = max_rect.max.x - animation_offset;
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
                    ui.horizontal(|ui| {
                        ui.heading(egui::RichText::new("Queue").heading().underline());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            ui.add_space(5.);
                            if ui.button(egui::RichText::new("X")).clicked() {
                                to_send.extend_from_slice(&[
                                    shared::message::ClientMessage::Command(
                                        shared::command::PlayerCommand::ClearQueue.into(),
                                    ),
                                ]);
                            }
                        })
                    });
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
