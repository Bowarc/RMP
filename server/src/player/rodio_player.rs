use rodio::decoder;

pub struct RodioPlayer {
    config: crate::config::PlayerConfig,

    sink: rodio::Sink,

    queue: Vec<uuid::Uuid>,
}

impl RodioPlayer {
    pub fn new(config: crate::config::PlayerConfig) -> Result<Self, crate::error::PlayerError> {
        use {
            crate::error::PlayerError,
            rodio::{OutputStream, Sink},
        };
        Ok(Self {
            config,

            sink: {
                let (_stream, handle) =
                    OutputStream::try_default().map_err(|e| PlayerError::Initialisation {
                        name: "Rodio".to_string(),
                        error: e.to_string(),
                    })?;
                Sink::try_new(&handle).map_err(|e| PlayerError::Initialisation {
                    name: "Rodio".to_string(),
                    error: e.to_string(),
                })?
            },

            queue: vec![],
        })
    }
}

impl super::Player for RodioPlayer {
    fn play(&mut self) -> Result<(), crate::error::PlayerError> {
        use crate::error::PlayerError;
        use rodio::decoder::Decoder;

        if self.queue.is_empty() {
            return Err(PlayerError::EmptyStack);
        }

        let song_id = self.queue.first().unwrap(); // We checked before if the stack was empty

        unreachable!("Fix the extension before");
        let path = self.config.song_path().join(format!("{song_id}.?"));

        // let data = std::fs::read(path).map_err(|e| PlayerError::FileRead {
        //     target: path.to_str().to_owned(),
        //     error: e.to_string(),
        // })?;

        let file = std::fs::File::open(path).map_err(|e| PlayerError::FileRead {
            target: path.display().to_string(),
            error: e.to_string(),
        })?;

        let reader = std::io::BufReader::new(file);

        let decoder = Decoder::new(reader).map_err(|e| PlayerError::Initialisation {
            name: "Rodio".to_string(),
            error: format!(
                "Decoder initialisation failed on {} with: {}",
                song_id.as_hyphenated().to_string(),
                e.to_string()
            ),
        })?;

        self.sink.append(decoder);
        self.sink.play();
    }

    fn pause(&mut self) -> Result<(), crate::error::PlayerError> {
        todo!()
    }

    fn add_queue(&mut self, uuid: uuid::Uuid) -> Result<(), crate::error::PlayerError> {
        self.queue.push(uuid.clone());
        Ok(())
    }

    fn remove_queue(&mut self, uuid: uuid::Uuid) -> Result<(), crate::error::PlayerError> {
        let Some(index) = self.queue.iter().position(|item| *item == uuid) else {
            return Ok(());
        };

        self.queue.remove(index);
        Ok(())
    }

    fn update(&mut self) -> Result<(), crate::error::PlayerError> {
        if self.sink.empty() && !self.queue.is_empty() {
            let _ = self.play();
        }

        Ok(())
    }
}
