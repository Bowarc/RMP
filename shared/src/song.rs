#[derive(Debug, Clone, derivative::Derivative, serde::Serialize, serde::Deserialize)]
#[derivative(PartialEq)]
pub struct Song {
    uuid: uuid::Uuid, // No need to compare anything else
    #[derivative(PartialEq = "ignore")]
    metadata: Metadata,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    title: String,
    duration: std::time::Duration,
    // author: Option<String>
    // more ?
}

impl Song {
    pub fn new(uuid: uuid::Uuid, metadata: Metadata) -> Self {
        Self { uuid, metadata }
    }
    pub fn load(uuid: uuid::Uuid, song_folder_path: &std::path::Path) -> Option<Self> {
        let path = song_folder_path
            .canonicalize()
            .ok()?
            .join(uuid.as_hyphenated().to_string());

        let metadata = Metadata::load(&path)?;

        Some(Song { uuid, metadata })
    }

    pub fn data(
        &self,
        song_folder_path: &std::path::Path,
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
    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

impl Metadata {
    pub fn new(title: String, duration: std::time::Duration) -> Self {
        Self { title, duration }
    }

    fn load(path: &std::path::Path) -> Option<Self> {
        use ron::de::from_bytes;
        use std::io::Read as _;

        let mut bytes = Vec::with_capacity(std::mem::size_of::<Metadata>());

        let _ = std::fs::File::open(format!("{path}.metadata", path = path.display()))
            .ok()?
            .read_to_end(&mut bytes)
            .ok()?;

        from_bytes(&bytes).ok()
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn duration(&self) -> &std::time::Duration {
        &self.duration
    }
    pub fn write_to_file(&self, uuid: uuid::Uuid, songs_path: std::path::PathBuf) -> Result<(), crate::error::ImporterError> {
        use std::fs::OpenOptions;
        let path = {
            let mut p = songs_path;
            p.push(format!("{uuid}.metadata"));
            p
        };

        let file = OpenOptions::new().create_new(true).write(true).open(path)?;

        ron::ser::to_writer(file, self)?;

        Ok(())
    }
}

///
/// Tries to decode the local_file_path file, supported formats are:
///     MP3, WAV, flac and vorbis (I don't really plan on using the 2 lasts)
///
/// Creates {uuid} and {uuid}.metadata files in local_storage_path which should be something like ./songs
///
///     If it fails, it tries to cleanup
///
pub fn convert_local(
    local_file_path: std::path::PathBuf,
    // TODO: rename this to something closer to local library path, with #11
    local_storage_path: std::path::PathBuf,
) -> Option<Song> {
    use rodio::{decoder::Decoder, Source};
    use std::fs::OpenOptions;

    let uuid = uuid::Uuid::new_v4();

    static DEFAULT_NAME: &str = "DEFAULT_IMPORTED_FILE_NAME";

    let local_file_name = local_file_path
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| {
            if !s.contains(".") {
                return s;
            }

            if s.replace(".", "").is_empty() {
                return DEFAULT_NAME;
            };

            let dot_index = s.rfind(".").unwrap();

            &s[0..dot_index]
        })
        .unwrap_or(DEFAULT_NAME)
        .to_string();

    let meta_file_path = {
        let mut p = local_storage_path.clone();
        p.push(format!("{uuid}.metadata"));
        p
    };

    let data_file_path = {
        let mut p = local_storage_path.clone();
        p.push(uuid.as_hyphenated().to_string());
        p
    };

    let decoder = Decoder::new(std::fs::File::open(&local_file_path).ok()?)
        .inspect_err(|e| {
            error!("Invalid file format: {e}");
        })
        .ok()?;
    let duration = match decoder.total_duration() {
        Some(d) => d,
        None => {
            // This gives the duration, which is already not that bad, but i fear that it's the only thing this crate will bring
            let data = ffprobe::ffprobe(&local_file_path).unwrap();
            debug!("ffprobe data: {data:#?}");

            data.format.get_duration().unwrap()
        }
    };

    let metadata = Metadata::new(local_file_name.clone(), duration);
    metadata
        .write_to_file(uuid, local_storage_path.clone())
        .map_err(|e| {
            error!("Failed to write metadata due to: {e}");
        })
        .ok()?;

    // Write new data file for imported song (which is just a plain copy)
    let mut data_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&data_file_path)
        .map_err(|e| {
            let _ = std::fs::remove_file(&meta_file_path);
            error!("Failed to create data file with: {e}");
            e
        })
        .ok()?;

    let mut local_file = OpenOptions::new().read(true).open(local_file_path).unwrap(); // yea

    let _ = std::io::copy(&mut local_file, &mut data_file)
        .map_err(|e| {
            error!("Failed to copy to local storage due to: {e}");

            let _ = std::fs::remove_file(meta_file_path);
            let _ = std::fs::remove_file(data_file_path);
        })
        .ok()?;

    Some(Song::new(uuid, metadata))
}
