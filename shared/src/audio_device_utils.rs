pub fn list_host_devices() -> Vec<rodio::Device> {
    use rodio::cpal::{self, traits::HostTrait as _};

    // I have no guaranties that this list will stay the same throughout the execution of my program, do i ?
    cpal::default_host().output_devices().unwrap().collect()
    // Doesn't look like it..
    // can't even use that for id'ing devices ..

}

// Returns default if not found
pub fn get_device_by_name(
    target_device_name: &str,
) -> Result<
    (rodio::OutputStream, rodio::OutputStreamHandle),
    (rodio::OutputStream, rodio::OutputStreamHandle),
> {
    use rodio::{
        cpal::{
            self,
            traits::{DeviceTrait as _, HostTrait as _},
        },
        Device, OutputStream,
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

        if let Ok(s_sh) = OutputStream::try_from_device(&device) {
            return Ok(s_sh);
        }
    }

    Err(OutputStream::try_default().unwrap())
}
