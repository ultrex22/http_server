use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use crate::http;

// Request object //
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: http::Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

// ParseError object //
#[allow(clippy::enum_variant_names)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocal,
    InvalidMethod,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocal => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}
