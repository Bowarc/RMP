pub fn download(socket: &mut shared::Socket, url: String) {
    use shared::{
        command::{Command, DownloaderCommand},
        message::ClientMessage,
    };

    socket
        .send(ClientMessage::Command(Command::Downloader(
            DownloaderCommand::QueueDownload(url),
        )))
        .unwrap();
}

pub fn fetch_current(socket: &mut shared::Socket) -> Vec<shared::download::Report> {
    use shared::{
        command::{Command, DownloaderCommand},
        message::ServerMessage,
    };

    crate::send_and_wait!(socket, Command::Downloader(DownloaderCommand::FetchCurrent), Vec<shared::download::Report>, ServerMessage::CurrentDownloads(reports) => reports)
}
