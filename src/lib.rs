extern crate librealsense_sys as rs;

use std::result;

pub mod error;
pub mod context;
pub mod device;

pub fn version() -> String {
    format!("v{:?}.{:?}.{:?}",
            rs::RS_API_MAJOR_VERSION,
            rs::RS_API_MINOR_VERSION,
            rs::RS_API_PATCH_VERSION)
}

pub type Result<T> = result::Result<T, error::Error>;
