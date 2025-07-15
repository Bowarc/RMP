pub fn download(client: &mut shared::client::Client, url: String){
    use {
        shared::{
            command::{Command, DownloaderCommand},
            message::ClientMessage,
        }
    };

    client.send(
        ClientMessage::Command(
            Command::Downloader(DownloaderCommand::QueueDownload(url))
        )
    ).unwrap();

}
