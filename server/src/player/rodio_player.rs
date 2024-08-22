pub struct RodioPlayer {
    config: crate::config::PlayerConfig,

    sink: rodio::Sink,

    // This needs to be kept alive even tho it's not explicitly used,
    // see <https://docs.rs/rodio/latest/rodio/struct.OutputStream.html> and <https://github.com/Bowarc/RMP/issues/3> for more informaton
    _os: rodio::OutputStream,

    queue: Vec<shared::song::Song>,
    queue_pointer: Option<u64>,// I don't think we need more than u8 but eh
}

impl RodioPlayer {
    pub fn new(config: crate::config::PlayerConfig) -> Result<Self, crate::error::PlayerError> {
        use {
            crate::error::PlayerError,
            rodio::{DeviceTrait as _, OutputStream, Sink},
        };

        let devices = shared::audio_device_utils::list_host_devices()
            .iter()
            .map(|d| d.name().unwrap_or("UNKNOWN_NAME".to_string()))
            .map(|dn| format!("  {dn}\n"))
            .collect::<String>();

        debug!(
            "Creating a rodio player\nSong path: {}\nDevice list:[\n{devices}]",
            config.song_path().canonicalize().unwrap().display()
        );

        // Is it not working because im dropping that thing ?
        // Yes, see #3
        let (stream, handle) =
            OutputStream::try_default().map_err(|e| PlayerError::Initialisation {
                name: "Rodio".to_string(),
                error: e.to_string(),
            })?;

        // let (stream, handle) = match shared::audio_device_utils::get_device_by_name(
        //     "SteelSeries Sonar - Aux (SteelSeries Sonar Virtual Audio Device)",
        // ) {
        //     Ok(os_osh) => os_osh,
        //     Err(e_os_osh) => {
        //         warn!("Could not get the requested device, falling back to default.");
        //         e_os_osh
        //     }
        // };

        let mut sink = Sink::try_new(&handle).map_err(|e| PlayerError::Initialisation {
            name: "Rodio".to_string(),
            error: e.to_string(),
        })?;

        sink.pause(); // Not playing, why not paused ?

        Ok(Self {
            config,

            sink,

            _os: stream,

            queue: vec![],
            queue_pointer: None,
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
        debug!("Rodio player set to play !");
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

    fn set_device_by_name(&mut self, new_device_name: &str) -> super::Result<()> {
        use rodio::Sink;
        // Do you really need to create a new sink ? :c
        // well, let's try to get the most information about the current one, and replace it

        let Ok((os, osh)) = shared::audio_device_utils::get_device_by_name(&new_device_name)
        else {
            return Err(crate::error::PlayerError::Initialisation {
                name: "Rodio".to_string(),
                error: format!("Could not initialise requested device ({new_device_name})"),
            });
        };

        let Ok(new_sink) = Sink::try_new(&osh) else{
            panic!()
        };

        new_sink.pause(); // pause by default, see after

        new_sink.set_volume(self.sink.volume());

        if !self.sink.is_paused(){
            debug!("The current sink is playing, let's try to set the other one as playing too!");

            let current_pos = self.sink.get_pos();

            let old_sink = std::mem::replace(&mut self.sink, new_sink);
            let old_os = std::mem::replace(&mut self._os, os); // important

            if let Err(e) = self.play(){
                error!("{e}");

                self.sink = old_sink;
                self._os = old_os
            }

            self.set_pos(current_pos)?;

        }

        debug!("Successfully swapped output device to: {new_device_name}");

        Ok(())
    }

    fn set_pos(&mut self, pos: std::time::Duration) -> super::Result<()> {
        self.sink.try_seek(pos).map_err(|s_e|{
            crate::error::PlayerError::SeekError(s_e.to_string())
        })
    }

    fn pos(&self) -> super::Result<std::time::Duration> {
        Ok(self.sink.get_pos())

    }

    fn update(&mut self) -> super::Result<()> {
        // if self.sink.empty() && !self.queue.is_empty() {
        //     debug!("Sink is empty but queue isn't, setting sink to play mode !");
        //     let _ = self.play();
        // }

        Ok(())
    }
}
