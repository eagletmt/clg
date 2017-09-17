extern crate std;
extern crate url;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    UrlParseError(url::ParseError),
    Custom(String),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Error::Io(ref e) => e.fmt(f),
            Error::UrlParseError(e) => e.fmt(f),
            Error::Custom(ref msg) => write!(f, "{}", msg),
        }
    }
}
impl std::error::Error for Error {
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::UrlParseError(ref e) => Some(e),
            Error::Custom(_) => None,
        }
    }

    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::UrlParseError(ref e) => e.description(),
            Error::Custom(ref msg) => msg,
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::UrlParseError(e)
    }
}
