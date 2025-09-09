pub fn downloader_tab(
    ui: &mut egui::Ui,
    _ectx: &egui::Context,
    client: &mut client::Client,
    downloader_last_update_request: &mut std::time::Instant,
    polling_rate: &std::time::Duration,
) {
    let mut to_send = Vec::new();
    if downloader_last_update_request.elapsed().as_secs_f32() > polling_rate.as_secs_f32() * 1.1 {
        *downloader_last_update_request = std::time::Instant::now();
        to_send.push(shared::message::ClientMessage::Command(
            shared::command::DownloaderCommand::FetchCurrent.into(),
        ));
    }

    ui.group(|ui| {
        ui.label("Download Music");
        ui.horizontal(|ui| {
            ui.label("URL:");
            ui.text_edit_singleline(&mut client.downloader_data_mut().new_download_url);
            if ui.button("Download").clicked() {
                // Simulate adding a download
                to_send.push(shared::message::ClientMessage::Command(
                    shared::command::DownloaderCommand::QueueDownload(
                        client.downloader_data().new_download_url.clone(),
                    )
                    .into(),
                ));
                client.downloader_data_mut().new_download_url.clear(); // Clear the input field
            }
        });

        for download_report in client.downloader_data_mut().current_downloads.iter_mut() {
            ui.horizontal(|ui| {
                ui.label(&download_report.url);
                use shared::download::Phase;
                match &download_report.phase {
                    Phase::Waiting => {
                        ui.label("Waiting. . .");
                    }
                    Phase::PreProcessing => {
                        ui.label("Pre processing");
                    }
                    Phase::Downloading { current_percentage } => {
                        ui.set_min_width(12. * 3.);
                        ui.label(format!("{:.0}%", *current_percentage));
                        ui.add(
                            egui::widgets::ProgressBar::new(*current_percentage / 100.)
                                .desired_width(100.),
                        );
                    }
                    Phase::Postrocessing => {
                        ui.label("Post processing");
                    }
                    Phase::Done => {
                        ui.label("Done");
                    }
                    Phase::Error => {
                        ui.label("Failed");
                    }
                }
            });
        }
    });

    if let Err(e) = client.send_multiple(to_send) {
        error!("Failed to send a downloader message due to: {e}");
    }
}
