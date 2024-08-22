pub fn list_host_devices() -> Vec<rodio::cpal::Device> {
    use rodio::cpal::{self, traits::HostTrait as _};

    // I have no guaranties that this list will stay the same throughout the execution of my program, do i ?
    cpal::default_host().output_devices().unwrap().collect()
    // Doesn't look like it..
    // can't even use that for id'ing devices ..

    // well, let's hope names are consistant

}

// Returns default if not found
pub fn get_device_by_name(
    target_device_name: &str,
) -> Result<
    rodio::cpal::Device,
    rodio::cpal::Device,
> {
    use rodio::{
        cpal::{
            self,
            traits::{DeviceTrait as _, HostTrait as _},
        },
        Device,
    };

    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();

    for (i, device) in devices.map(Device::from).enumerate() {

        let Ok(device_name) = device.name() else {
            warn!("Failed to fetch the device name of device #{i}");
            continue;
        };

        if device_name != target_device_name {
            continue;
        }

        return Ok(device)
    }

    Err(get_default_device())
}

#[inline]
pub fn get_default_device() -> rodio::cpal::Device{
    use rodio::cpal::traits::HostTrait as _;
    rodio::cpal::default_host().default_output_device().unwrap()
}