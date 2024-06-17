use std::{io, num::ParseFloatError};

use chrono::ParseError;

#[derive(Debug)]
pub enum TruckError {
    BadTimeframe(TimeFrameError),
}

#[derive(Debug)]
pub enum TimeFrameError {
    BadDataPoint(String, DataPointErr),
}

#[derive(Debug)]
pub enum DataPointErr {
    InsufficientEntries,
    BadDate(ParseError),
    BadCoord(ParseFloatError),
}

#[derive(Debug)]
pub enum DataParseError {
    MalformedEntry(String),
    IoError(io::Error),
    TruckError(String, TruckError),
}

impl From<io::Error> for DataParseError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseError> for DataPointErr {
    fn from(value: ParseError) -> Self {
        Self::BadDate(value)
    }
}

impl From<ParseFloatError> for DataPointErr {
    fn from(value: ParseFloatError) -> Self {
        Self::BadCoord(value)
    }
}
