use std::{fmt, error};
use rs;

#[derive(Debug)]
pub struct Error {
    raw: *mut rs::rs_error,
}

impl Error {
    pub fn new() -> Error {
        Error { raw: Box::into_raw(Box::new(rs::rs_error::new())) as *mut rs::rs_error }
    }

    pub fn inner(&self) -> *mut rs::rs_error {
        self.raw
    }

    pub fn message(&self) -> String {
        // TODO(@benhinchley): return error message from rs::rs_get_error_message(error: *const rs_error)
        unimplemented!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "librealsense error")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        // TODO(@benhinchley): implment this function
        // need to get string from this function
        // rs_get_error_message(self)
        // and create a &str that can survive the lifetime
        unimplemented!()
    }
}
