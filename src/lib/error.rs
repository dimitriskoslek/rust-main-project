use std::fmt;
use std::io;

pub enum SocError {
    IoError(io::Error),
    //NoDirectory,
    //InvalidCommit,
    //InvalidIndex,
}

impl fmt::Display for SocError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &SocError::IoError(ref e) => e.fmt(formatter),
            //&SocError::NoDirectory => formatter.write_str("No directory found"),
        }
    }
}

impl From<io::Error> for SocError {
    fn from(err: io::Error) -> SocError {
        SocError::IoError(err)
    }
}