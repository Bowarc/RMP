pub fn download(socket: &mut shared::Socket, url: String){
    use {
        shared::{
            command::{Command, DownloaderCommand},
            message::ClientMessage,
        }
    };

    socket.send(
        ClientMessage::Command(
            Command::Downloader(DownloaderCommand::QueueDownload(url))
        )
    ).unwrap();

}
