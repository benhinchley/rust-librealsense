use rs;
use error::Error;
use Result;

pub struct Device {
    raw: *const rs::rs_device,
}

impl Device {
    pub fn new(dev: *const rs::rs_device) -> Device {
        Device { raw: dev }
    }
}
