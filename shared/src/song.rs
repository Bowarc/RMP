#[derive(Debug, Clone, derivative::Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(PartialEq)]
pub struct Song {
    uuid: uuid::Uuid, // No need to compare anything else
    #[derivative(PartialEq = "ignore")]
    metadata: Metadata,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    name: String,
    duration: std::time::Duration,
    // author: Option<String>
    // more ?
}

impl Song {
    pub fn new(uuid: uuid::Uuid, metadata: Metadata) -> Self {
        Self { uuid, metadata }
    }
    pub fn load(uuid: uuid::Uuid, song_folder_path: &std::path::PathBuf) -> Option<Self> {
        let path = song_folder_path
            .canonicalize()
            .ok()?
            .join(uuid.as_hyphenated().to_string());

        let metadata = Metadata::load(&path)?;

        Some(Song { uuid, metadata })
    }

    pub fn data(
        &self,
        song_folder_path: &std::path::PathBuf,
    ) -> Option<std::io::BufReader<std::fs::File>> {
        let path = song_folder_path
            .canonicalize()
            .ok()?
            .join(self.uuid.as_hyphenated().to_string());

        let file = std::fs::File::open(path).ok()?;

        Some(std::io::BufReader::new(file))
    }

    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }
}

impl Metadata {
    pub fn new(name: String, duration: std::time::Duration) -> Self {
        Self { name, duration }
    }

    fn load(path: &std::path::PathBuf) -> Option<Self> {
        use ron::de::from_bytes;
        use std::io::Read as _;

        let mut bytes = Vec::with_capacity(std::mem::size_of::<Metadata>());

        let _ = std::fs::File::open(format!("{path}.metadata", path = path.display()))
            .ok()?
            .read_to_end(&mut bytes)
            .ok()?;

        from_bytes(&bytes).ok()
    }
}


///
///
/// Tries to decode the local_file_path file, supported formats are:
///     MP3, WAV, flac and vorbis (I don't really plan on using the 2 lasts)
///
/// Creates {uuid} and {uuid}.metadata files in local_storage_path which should be something like ./songs
///
///     If it fails, it tries to cleanup
///
///
pub fn convert_local(
    local_file_path: std::path::PathBuf,
    local_storage_path: std::path::PathBuf,
) -> Option<Song> {
    use rodio::decoder::Decoder;
    use std::fs::File;
    use std::io::{Read as _, Write as _};

    let uuid = uuid::Uuid::new_v4();

    static DEFAULT_NAME: &'static str = "DEFAULT_IMPORTED_FILE_NAME";

    let local_file_name = local_file_path
        .file_name()
        .map(|s| s.to_str())
        .flatten()
        .map(|s| {
            if !s.contains(".") {
                return s;
            }

            if s.replace(".", "").len() ==0{
                return DEFAULT_NAME;
            };

            let dot_index = s.rfind(".").unwrap();

            &s[0..dot_index]
        })
        .unwrap_or(DEFAULT_NAME)
        .to_string();

    let meta_file_path = format!(
        "{p}/{uuid}.metadata",
        p = local_storage_path.display()
    );
    let data_file_path =format!("{p}/{uuid}", p = local_storage_path.display());

    Decoder::new(std::fs::File::open(&local_file_path).ok()?)
        .map_err(|e| {
            error!("Invalid file format");
            e
        })
        .ok()?;

    // This gives the duration, which is already not that bad, but i fear that it's the only thing this crate will bring
    let data = ffprobe::ffprobe(&local_file_path).unwrap();

    debug!("ffprobe data: {data:#?}");

    // Write metadata for imported song
    let metadata_file = File::create_new(&meta_file_path)
    .map_err(|e| {
        error!("Failed to create metadata file with: {e}");
        e
    })
    .ok()?;

    let metadata = Metadata::new(
        local_file_name.clone(),
        data.format
            .get_duration()
            .unwrap_or(std::time::Duration::from_secs(1)),
    );

    if let Err(e) = ron::ser::to_writer(metadata_file, &metadata) {
        error!("Could not write metadata file due to: {e}");
        return None;
    }

    // Write new data file for imported song (which is just a plain copy)
    let mut data_file = File::create_new(&data_file_path)
        .map_err(|e| {
            let _ = std::fs::remove_file(&meta_file_path);
            error!("Failed to create data file with: {e}");
            e
        })
        .ok()?;

    let mut local_file = File::open(local_file_path).unwrap(); // yea

    let mut local_buffer = Vec::new();

    if let Err(e) = local_file.read_to_end(&mut local_buffer) {
        error!("Could not read local file due to: {e}");
        return None;
    }

    if let Err(e) = data_file.write_all(&local_buffer) {
        error!("Could not write data file due to: {e}");
        // TODO: Cleanup of actions done above
        let _ = std::fs::remove_file(meta_file_path);
        let _ = std::fs::remove_file(data_file_path);
        return None;
    }

    Some(Song::new(uuid, metadata))
}
