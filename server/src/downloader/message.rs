#[derive(Debug, PartialEq)]
pub enum DownloadMessage {
    PrcentageUpdate(f32),
    ChunkUpdate(Vec<u8>),
    Error(String), // every error means that the downloader has exited
    Done,
}