use std::ops::Drop;

use rs;
use Result;
use error::Error;
use device::Device;

#[derive(Debug)]
pub struct Context {
    raw: *const rs::rs_context,
}

impl Context {
    pub fn new() -> Result<Context> {
        unsafe {
            let err = Error::new();
            let api_version: i32 = rs::RS_API_VERSION as i32;
            let ctx = rs::rs_create_context(api_version, &mut err.inner() as *mut _);

            // TODO(@benhinchley): actually handle the potential error case
            Ok(Context { raw: ctx })
        }
    }

    pub fn get_device_count(self) -> Result<i32> {
        unsafe {
            let err = Error::new();
            let count: i32 = rs::rs_get_device_count(self.raw, &mut err.inner() as *mut _) as i32;

            // TODO(@benhinchley): actually handle the potential error case
            Ok(count)
        }
    }

    pub fn get_device(self, idx: i32) -> Result<Device> {
        unsafe {
            let err = Error::new();
            let dev = rs::rs_get_device(self.raw as *mut _,
                                        idx, // as c_int
                                        &mut err.inner() as *mut _);

            // TODO(@benhinchley): actually handle the potential error case
            Ok(Device::new(dev))
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            let err = Error::new();
            rs::rs_delete_context(self.raw as *mut _, &mut err.inner() as *mut _)
        }
    }
}

#[test]
fn new_context() {
    match Context::new() {
        Ok(_) => (),
        Err(_) => panic!(),
    }
}
