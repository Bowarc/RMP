use egui::Slider;

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

    // TODO: Fix that not being a const
    #[allow(non_snake_case)]
    let BOTTOM_BAR_HEIGHT_MEMORY_ID = "bottom_bar_height".into();

    let mut to_send = Vec::new();

    let rect = {
        let mut r = rect;
        if let Some(saved) = ui.memory(|mem| mem.data.get_temp::<f32>(BOTTOM_BAR_HEIGHT_MEMORY_ID))
        {
            r.min.y = rect.max.y - saved;
        }
        r
    };

    let ui_width = rect.width();
    let ui_width_third = ui_width * 0.3;

    let left_rect = {
        let mut r = rect;
        r.max.x = r.min.x + ui_width_third;
        r
    };

    let mid_rect = {
        let mut r = rect;
        r.min.x += ui_width_third;
        r.max.x -= ui_width_third;
        r
    };

    let right_rect = {
        let mut r = rect;
        r.min.x = r.max.x - ui_width_third;
        r
    };

    println!("{left_rect}, {mid_rect}, {right_rect}");
    println!("{}, {}, {}",left_rect.width(), mid_rect.width(),right_rect.width());

    let left_resp = ui.scope_builder(
        UiBuilder::new()
            .max_rect(left_rect)
            .layout(egui::Layout::left_to_right(egui::Align::TOP)),
        |ui| {
            ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
            ui.label(song.metadata().title());
        },
    );

    let mid_resp = ui.scope_builder(UiBuilder::new().max_rect(mid_rect), |ui| {
        ui.vertical_centered(|ui| {
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
        });
        ui.add_space(10.);

        ui.horizontal(|ui| {
            ui.style_mut().spacing.slider_width = ui.max_rect().width() * 0.8;

            let mut p = client.player_data().position.as_secs_f32();

            ui.label(time_format(&Duration::from_secs_f32(p)));

            let resp = ui.add(
                egui::Slider::new(&mut p, 0.0..=song.metadata().duration().as_secs_f32())
                    .show_value(false)
                    .trailing_fill(true),
            );
            ui.label(time_format(song.metadata().duration()));
            if resp.changed() {
                to_send.push(shared::message::ClientMessage::Command(
                    shared::command::PlayerCommand::SetPosition(Duration::from_secs_f32(p)).into(),
                ));
            }
        });
    });

    let right_resp = ui.scope_builder(
        UiBuilder::new()
            .max_rect(right_rect)
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

    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a player message due to: {e}");
    }

    let bottom_bar_max_height = left_resp
        .response
        .rect
        .height()
        .max(mid_resp.response.rect.height())
        .max(right_resp.response.rect.height());

    ui.memory_mut(|mem| {
        mem.data
            .insert_temp(BOTTOM_BAR_HEIGHT_MEMORY_ID, bottom_bar_max_height);
    })
}

fn time_format(t: &std::time::Duration) -> String {
    let mut prec = 2;
    {
        const NANOS_IN_MICROSECOND: f64 = 1_000.0;
        const NANOS_IN_MILLISECOND: f64 = 1_000_000.0;
        const NANOS_IN_SECOND: f64 = 1_000_000_000.0;
        const NANOS_IN_MINUTE: f64 = NANOS_IN_SECOND * 60.0;
        const NANOS_IN_HOUR: f64 = NANOS_IN_MINUTE * 60.0;
        const NANOS_IN_DAY: f64 = NANOS_IN_HOUR * 24.0;
        const NANOS_IN_WEEK: f64 = NANOS_IN_DAY * 7.0;
        const NANOS_IN_YEAR: f64 = NANOS_IN_DAY * 365.0;

        let total_nanos = t.as_nanos() as f64;

        if total_nanos < 1.0 {
            return format!("{:.0}: ", total_nanos.floor());
        }

        let mut remaining_nanos = total_nanos;
        let mut formatted_duration = String::new();

        if remaining_nanos >= NANOS_IN_YEAR && prec != 0 {
            prec -= 1;
            let years = remaining_nanos / NANOS_IN_YEAR;
            formatted_duration.push_str(&format!("{:.0}: ", years.floor()));
            remaining_nanos %= NANOS_IN_YEAR;
        }

        if remaining_nanos >= NANOS_IN_WEEK && prec != 0 {
            prec -= 1;
            let weeks = remaining_nanos / NANOS_IN_WEEK;
            formatted_duration.push_str(&format!("{:.0}: ", weeks.floor()));
            remaining_nanos %= NANOS_IN_WEEK;
        }

        if remaining_nanos >= NANOS_IN_DAY && prec != 0 {
            prec -= 1;
            let days = remaining_nanos / NANOS_IN_DAY;
            formatted_duration.push_str(&format!("{:.0}: ", days.floor()));
            remaining_nanos %= NANOS_IN_DAY;
        }

        if remaining_nanos >= NANOS_IN_HOUR && prec != 0 {
            prec -= 1;
            let hours = remaining_nanos / NANOS_IN_HOUR;
            formatted_duration.push_str(&format!("{:.0}: ", hours.floor()));
            remaining_nanos %= NANOS_IN_HOUR;
        }

        if remaining_nanos >= NANOS_IN_MINUTE && prec != 0 {
            prec -= 1;
            let minutes = remaining_nanos / NANOS_IN_MINUTE;
            formatted_duration.push_str(&format!("{:.0}: ", minutes.floor()));
            remaining_nanos %= NANOS_IN_MINUTE;
        }

        if remaining_nanos >= NANOS_IN_SECOND && prec != 0 {
            prec -= 1;
            let seconds = remaining_nanos / NANOS_IN_SECOND;
            formatted_duration.push_str(&format!("{:.0}: ", seconds.floor()));
            remaining_nanos %= NANOS_IN_SECOND;
        }

        if remaining_nanos >= NANOS_IN_MILLISECOND && prec != 0 {
            prec -= 1;
            let milis = remaining_nanos / NANOS_IN_MILLISECOND;
            formatted_duration.push_str(&format!("{:.0}: ", milis.floor()));
            remaining_nanos %= NANOS_IN_MILLISECOND;
        }

        if remaining_nanos >= NANOS_IN_MICROSECOND && prec != 0 {
            prec -= 1;
            let micro = remaining_nanos / NANOS_IN_MICROSECOND;
            formatted_duration.push_str(&format!("{:.0}: ", micro.floor()));
            remaining_nanos %= NANOS_IN_MICROSECOND;
        }

        if remaining_nanos > 0.0 && prec != 0 {
            formatted_duration.push_str(&format!("{:.0}: ", remaining_nanos.floor()));
        }

        formatted_duration.trim().trim_end_matches(":").to_string()
    }
}
