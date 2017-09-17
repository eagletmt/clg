extern crate clap;
extern crate env_logger;
extern crate url;

#[macro_use]
extern crate log;

#[derive(Debug)]
enum Error {
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

fn main() {
    env_logger::init().unwrap();

    let app = clap::App::new("clg")
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap::SubCommand::with_name("clone").arg(
            clap::Arg::with_name("URL").required(true).takes_value(true),
        ));

    let result = match app.get_matches().subcommand() {
        ("clone", Some(submatch)) => clone(submatch),
        _ => unreachable!(),
    };
    match result {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}

fn clone(matches: &clap::ArgMatches) -> Result<i32, Error> {
    let arg = matches.value_of("URL").unwrap();
    let uri = parse_git_url(arg)?;
    let path = match std::env::home_dir() {
        Some(mut pathbuf) => {
            pathbuf.push(".ghq"); // TODO: Make customizable
            pathbuf.push(uri.host_str().unwrap());
            for c in std::path::PathBuf::from(uri.path()).components().skip(1) {
                pathbuf.push(c.as_os_str());
            }
            pathbuf
        }
        None => {
            return Err(Error::Custom("Cannot get HOME directory".to_owned()));
        }
    };
    debug!("Clone {} to {}", uri, path.display());
    let mut child = std::process::Command::new("git")
        .arg("clone")
        .arg(arg)
        .arg(&path)
        .spawn()?;
    let status = child.wait()?;
    Ok(status.code().unwrap_or(1))
}

fn parse_git_url(u: &str) -> Result<url::Url, Error> {
    // TODO: Support [user@]host.xz:path/to/repo.git form
    // https://git-scm.com/docs/git-push#_git_urls_a_id_urls_a
    let url = url::Url::parse("https://github.com").unwrap().join(u)?;
    Ok(url)
}
