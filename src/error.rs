use std::fmt;

pub enum PaynymError {
    Checksum
}

impl fmt::Display for PaynymError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PaynymError::Checksum => write!(f, "the paynym code does not match the checksum. Incorrect Paynym!")
        }
    }  
}

impl fmt::Debug for PaynymError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &dyn fmt::Display).fmt(f)
    }
}