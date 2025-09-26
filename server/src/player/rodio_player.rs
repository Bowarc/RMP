use rodio::DeviceTrait;

pub struct RodioPlayer {
    config: crate::config::PlayerConfig,

    sink: rodio::Sink,

    // This needs to be kept alive even tho it's not explicitly used,
    // see <https://docs.rs/rodio/latest/rodio/struct.OutputStream.html> and <https://github.com/Bowarc/RMP/issues/3> for more informaton
    _os: rodio::OutputStream,

    current_device: rodio::cpal::Device,

    queue: Vec<shared::song::Song>,
    queue_pointer: Option<u64>, // I don't think we need more than u8 but eh
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
            "Creating a rodio player\nSong path: {}\nDevice list: [\n{devices}]",
            config.song_path().canonicalize().unwrap().display()
        );

        // Is it not working because im dropping that thing ?
        // Yes, see #3
        // let (stream, handle) =
        //     OutputStream::try_default().map_err(|e| PlayerError::Initialisation {
        //         name: "Rodio".to_string(),
        //         error: e.to_string(),
        //     })?;

        // let device = match shared::audio_device_utils::get_device_by_name(
        //     "SteelSeries Sonar - Aux (SteelSeries Sonar Virtual Audio Device)",
        // ) {
        //     Ok(device) => device,
        //     Err(default_device) => default_device,
        // };

        let device = shared::audio_device_utils::get_default_device();

        let (stream, handle) =
            OutputStream::try_from_device(&device).map_err(|e| PlayerError::Initialisation {
                name: "Rodio".to_string(),
                error: format!(
                    "Could not create an OutputStream from device: {name}, due to: {e}",
                    name = device.name().unwrap_or("UNKNOWN_NAME".to_string())
                ),
            })?;

        let sink = Sink::try_new(&handle).map_err(|e| PlayerError::Initialisation {
            name: "Rodio".to_string(),
            error: e.to_string(),
        })?;

        sink.pause(); // Not playing, why not paused ?
        sink.set_volume(config.volume());

        Ok(Self {
            config,

            sink,

            _os: stream,

            current_device: device,

            queue: vec![],
            queue_pointer: None,
        })
    }
}

impl super::Player for RodioPlayer {
    fn play(&mut self) -> super::Result<()> {
        use crate::error::PlayerError;
        use rodio::decoder::Decoder;

        if self.sink.len() != 0 {
            // Already playing
            return Ok(());
        }

        if self.queue.is_empty() {
            return Err(PlayerError::EmptyQueue);
        }

        if self.queue_pointer.is_none() {
            self.queue_pointer = Some(0)
        }

        // queue pointer has litterally been sent just above, unwrap is fine
        let qp = self.queue_pointer.as_mut().unwrap();

        // If queue pointer is outside queue, let's bring it back
        // No, return an error !
        if *qp >= self.queue.len() as u64 {
            // *qp = (self.queue.len() -1) as u64;
            return Err(PlayerError::Custom("End of queue".to_string()));
        }

        let current_song = self.queue.get(*qp as usize).unwrap(); // This should never happend, we checked if the list was long enough just before

        debug!("{:?}", self.config.song_path());
        let Some(reader) = current_song.data(self.config.song_path()) else {
            return Err(crate::error::PlayerError::FileRead {
                target: current_song.uuid().as_hyphenated().to_string(),
                error: String::from("Could not read the data file"),
            });
        };

        let decoder = Decoder::new(reader).map_err(|e| PlayerError::Initialisation {
            name: "Rodio".to_string(),
            error: format!(
                "Decoder initialisation failed on {} with: {}",
                current_song.uuid().as_hyphenated(),
                e
            ),
        })?;

        self.sink.append(decoder);
        self.sink.play();

        debug!("Rodio player set to play !");
        Ok(())
    }

    fn pause(&mut self) -> super::Result<()> {
        self.sink.pause();
        Ok(())
    }

    fn is_playing(&self) -> bool {
        !self.sink.is_paused()
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

    fn remove_queue(&mut self, index: u16) -> super::Result<()> {
        use std::cmp::Ordering;

        let old = self.queue.remove(index as usize);

        // if self.queue.is_empty(){
        //     self.queue_pointer = None;
        // }

        let mut should_play = false;
        if let Some(qp) = &mut self.queue_pointer {
            match (*qp).cmp(&(index as u64)) {
                Ordering::Greater => {
                    // Since the queue pointer is further away in the queue, removing one will cause a song to be skipped
                    *qp -= 1
                }
                Ordering::Equal => {
                    // Removing the currently playing song should cause the player to stop
                    self.sink.clear();
                    should_play = true;
                }
                Ordering::Less => {
                    // The queue pointer is before the deleted song in the list, no need to change anything
                }
            }

            // If the queue pointer is outside the queue, let's bring it back
            if *qp >= self.queue.len() as u64 {
                *qp = (self.queue.len() - 1) as u64
            }
        }
        if should_play {
            self.play()?
        }

        debug!("{} has been remove from rodio player's queue !", old.uuid());
        Ok(())
    }

    fn clear_queue(&mut self) -> super::Result<()> {
        self.queue.clear();
        self.sink.clear();
        self.queue_pointer = None;
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

    fn audio_device(&self) -> super::Result<String> {
        self.current_device
            .name()
            .map_err(|e| crate::error::PlayerError::DeviceError {
                name: "Rodio".to_string(),
                e: format!("Could not get the name for the current device due to: {e}"),
            })
    }

    fn set_device_by_name(&mut self, new_device_name: &str) -> super::Result<()> {
        use rodio::Sink;

        let device =
            shared::audio_device_utils::get_device_by_name(new_device_name).map_err(|_| {
                crate::error::PlayerError::Initialisation {
                    name: "Rodio".to_string(),
                    error: format!("Could not get the requested device ({new_device_name})"),
                }
            })?;

        let Ok((os, osh)) = rodio::OutputStream::try_from_device(&device) else {
            return Err(crate::error::PlayerError::Initialisation {
                name: "Rodio".to_string(),
                error: format!(
                    "Could not build an outputstream from requested device ({new_device_name})"
                ),
            });
        };

        let Ok(new_sink) = Sink::try_new(&osh) else {
            panic!()
        };

        new_sink.pause(); // pause by default, see after

        new_sink.set_volume(self.sink.volume());

        if !self.sink.is_paused() {
            debug!("The current sink is playing, let's try to set the other one as playing too!");

            let current_pos = self.sink.get_pos();

            let old_sink = std::mem::replace(&mut self.sink, new_sink);
            let old_os = std::mem::replace(&mut self._os, os); // important

            if let Err(e) = self.play() {
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
        self.sink
            .try_seek(pos)
            .map_err(|s_e| crate::error::PlayerError::SeekError(s_e.to_string()))
    }

    fn pos(&self) -> super::Result<std::time::Duration> {
        Ok(self.sink.get_pos())
    }

    fn currently_playing_index(&self) -> super::Result<u64> {
        self.queue_pointer
            .ok_or(crate::error::PlayerError::NotPlaying)
    }

    fn update(&mut self) -> super::Result<()> {
        // Autoplay
        if self.sink.len() == 0 {
            self.pause()?
        }

        // should be fine as the order of operation should optimize the unwrap condition out if the 2nd cond is not met
        if self.sink.len() == 0
            && let Some(qp) = &mut self.queue_pointer
            && *qp as usize != self.queue.len() - 1
        {
            *qp += 1;
            self.play()?
        }

        Ok(())
    }
}
