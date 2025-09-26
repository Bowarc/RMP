pub fn render(
    ui: &mut egui::Ui,
    ectx: &egui::Context,
    client: &mut client::Client,
) {
    let mut to_send = Vec::new();


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
    });

    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a player message due to: {e}");
    }
}
