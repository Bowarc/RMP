pub struct RodioPlayer {
    config: crate::config::PlayerConfig,

    sink: rodio::Sink,
    os: rodio::OutputStream,

    queue: Vec<shared::song::Song>,
}

impl RodioPlayer {
    pub fn new(config: crate::config::PlayerConfig) -> Result<Self, crate::error::PlayerError> {
        use {
            crate::error::PlayerError,
            rodio::{OutputStream, Sink},
        };

        debug!(
            "Creating a rodio player with song path: {}",
            config.song_path().canonicalize().unwrap().display()
        );
        let (stream /* Is it not working because im dropping that thing ?*/, handle) =
            OutputStream::try_default().map_err(|e| PlayerError::Initialisation {
                name: "Rodio".to_string(),
                error: e.to_string(),
            })?;

        use rodio::cpal;
        use rodio::cpal::traits::{HostTrait,DeviceTrait};

        cpal::default_host()
            .output_devices()
            .unwrap()
            .for_each(|device| {
                let dev: rodio::Device = device.into();
                let devName: String = dev.name().unwrap();
                debug!(" # Device : {}", devName);
            });

        let sink =  Sink::try_new(&handle).map_err(|e| PlayerError::Initialisation {
            name: "Rodio".to_string(),
            error: e.to_string(),
        })?;

        Ok(Self {
            config,

            sink,
            os: stream,

            queue: vec![],
        })
    }
}

impl super::Player for RodioPlayer {
    fn play(&mut self) -> super::Result<()> {
        use crate::error::PlayerError;
        use rodio::decoder::Decoder;

        if self.queue.is_empty() {
            return Err(PlayerError::EmptyStack);
        }

        let song = self.queue.first().unwrap(); // We checked before if the stack was empty

        // let path = self.config.song_path().join(song_id.as_hyphenated().to_string()); // no extension since we don't care about the file format, everything is handled by the decoder

        // // let data = std::fs::read(path).map_err(|e| PlayerError::FileRead {
        // //     target: path.to_str().to_owned(),
        // //     error: e.to_string(),
        // // })?;

        // let file = std::fs::File::open(path.clone()).map_err(|e| PlayerError::FileRead {
        //     target: path.display().to_string(),
        //     error: e.to_string(),
        // })?;

        let Some(reader) = song.data(self.config.song_path()) else {
            return Err(crate::error::PlayerError::FileRead {
                target: song.uuid().as_hyphenated().to_string(),
                error: String::from("Could not read the file"),
            });
        };

        let decoder = Decoder::new(reader).map_err(|e| PlayerError::Initialisation {
            name: "Rodio".to_string(),
            error: format!(
                "Decoder initialisation failed on {} with: {}",
                song.uuid().as_hyphenated().to_string(),
                e.to_string()
            ),
        })?;

        self.sink.append(decoder);
        self.sink.play();
        // std::thread::sleep(std::time::Duration::from_secs(5));
        debug!("Rodio player set to play !");
        debug!("{}", self.sink.is_paused());
        Ok(())
    }

    fn pause(&mut self) -> super::Result<()> {
        self.sink.pause();
        debug!("Rodio player paused !");
        Ok(())
    }

    fn add_queue(&mut self, uuid: uuid::Uuid) -> super::Result<()> {
        let Some(song) = shared::song::Song::load(uuid, self.config.song_path()) else {
            return Err(crate::error::PlayerError::FileRead {
                target: uuid.as_hyphenated().to_string(),
                error: String::from("Could not open requested file"),
            });
        };

        self.queue.push(song);

        debug!("New song added to rodio player's queue ({uuid}) !");
        Ok(())
    }

    fn remove_queue(&mut self, uuid: uuid::Uuid) -> super::Result<()> {
        let Some(index) = self.queue.iter().position(|item| item.uuid() == uuid) else {
            return Ok(());
        };

        self.queue.remove(index);

        debug!("{uuid} has been remove from rodio player's queue !");
        Ok(())
    }

    fn clear_queue(&mut self) -> super::Result<()> {
        self.queue.clear();
        self.sink.clear();
        debug!("Rodio player's queue cleared !");
        Ok(())
    }

    fn queue(&self) -> super::Result<Vec<shared::song::Song>> {
        debug!("Player's queue has been fetched !");
        Ok(self.queue.clone())
    }

    fn set_volume(&mut self, val: f32) -> super::Result<()> {
        self.sink.set_volume(val);
        debug!("Rodio player's volume changed to {val} !");
        Ok(())
    }

    fn volume(&self) -> super::Result<f32> {
        debug!("Rodio player's volume has been fetched !");

        Ok(self.sink.volume())
    }

    fn update(&mut self) -> super::Result<()> {
        if self.sink.empty() && !self.queue.is_empty() {
            let _ = self.play();
        }

        Ok(())
    }
}
