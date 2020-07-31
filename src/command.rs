pub fn clone(matches: &clap::ArgMatches) -> Result<i32, Box<dyn std::error::Error>> {
    let name = matches.value_of("name");
    let arg = matches.value_of("URL").unwrap();
    let config = super::Config::load_from_file();
    let uri = parse_git_url(arg)?;
    let path = destination_path_for(&config, &uri, name)?;
    log::debug!("Clone {} to {}", uri, path.display());
    let mut child = std::process::Command::new("git")
        .arg("clone")
        .arg(uri.into_string())
        .arg(&path)
        .spawn()?;
    let status = child.wait()?;
    Ok(status.code().unwrap_or(1))
}

pub fn look(matches: &clap::ArgMatches) -> Result<i32, Box<dyn std::error::Error>> {
    let repository = matches.value_of("REPOSITORY").unwrap();
    let config = super::Config::load_from_file();
    let mut local_repos = vec![];
    visit_local_repositories(&config.root, &mut |path| {
        if path.ends_with(repository) {
            local_repos.push(path.to_path_buf());
        }
    })?;
    if local_repos.is_empty() {
        eprintln!("No repository found matching {}", repository);
        Ok(1)
    } else if local_repos.len() == 1 {
        exec_shell(&local_repos[0])
    } else {
        eprintln!(
            "{} repositories found matching {}",
            local_repos.len(),
            repository
        );
        for r in local_repos {
            eprintln!("  - {}", r.display());
        }
        Ok(1)
    }
}

pub fn list(matches: &clap::ArgMatches) -> Result<i32, Box<dyn std::error::Error>> {
    let config = super::Config::load_from_file();
    if matches.is_present("completion") {
        visit_local_repositories(&config.root, &mut |path| {
            let host_and_path = path.strip_prefix(&config.root).unwrap();
            let repo = format!(
                "{}",
                std::path::Path::new(host_and_path.file_name().unwrap()).display()
            );
            println!("{}", repo);
            if let Some(host_and_user) = host_and_path.parent() {
                let user = host_and_user.file_name().unwrap();
                if let Some(user) = user.to_str() {
                    if !user.contains('.') {
                        println!("{}/{}", std::path::Path::new(user).display(), repo);
                    }
                }
            }
        })?;
    } else {
        visit_local_repositories(&config.root, &mut |path| {
            println!("{}", path.strip_prefix(&config.root).unwrap().display());
        })?;
    }
    Ok(0)
}

pub fn root(_: &clap::ArgMatches) -> Result<i32, Box<dyn std::error::Error>> {
    let config = super::Config::load_from_file();
    println!("{}", config.root.display());
    Ok(0)
}

fn parse_git_url(u: &str) -> Result<url::Url, Box<dyn std::error::Error>> {
    // https://git-scm.com/docs/git-push#_git_urls_a_id_urls_a
    match url::Url::parse(u) {
        Ok(uri) => {
            log::debug!("{} is absolute URI", u);
            Ok(uri)
        }
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            // Try scp-like URL
            if let Some(colon_idx) = u.find(':') {
                match u.find('/') {
                    Some(slash_idx) => {
                        if slash_idx > colon_idx {
                            return parse_scp_like_url(u, colon_idx);
                        }
                    }
                    None => {
                        return parse_scp_like_url(u, colon_idx);
                    }
                }
            }
            log::debug!("{} is GitHub.com URI", u);
            // Map :user/:repo to https://github.com/:user/:repo
            Ok(url::Url::parse("https://github.com").unwrap().join(u)?)
        }
        Err(e) => Err(e)?,
    }
}

fn parse_scp_like_url(u: &str, colon_idx: usize) -> Result<url::Url, Box<dyn std::error::Error>> {
    log::debug!("{} is scp-like URI", u);
    let user_and_host = &u[..colon_idx];
    let path = &u[colon_idx + 1..];
    Ok(url::Url::parse(&format!(
        "ssh://{}/{}",
        user_and_host, path
    ))?)
}

fn destination_path_for(
    config: &super::Config,
    uri: &url::Url,
    name: Option<&str>,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut pathbuf = config.root.clone();
    pathbuf.push(uri.host_str().unwrap());
    for c in std::path::PathBuf::from(uri.path()).components().skip(1) {
        pathbuf.push(c.as_os_str());
    }
    if let Some(name) = name {
        pathbuf.pop();
        pathbuf.push(name);
    }
    if pathbuf.extension() == Some(std::ffi::OsStr::new("git")) {
        pathbuf.set_extension("");
    }
    Ok(pathbuf)
}

fn visit_local_repositories<P, F>(dir: P, callback: &mut F) -> Result<(), std::io::Error>
where
    P: AsRef<std::path::Path>,
    F: FnMut(&std::path::Path),
{
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let git_dir = path.join(".git");
            if git_dir.is_dir() {
                callback(&path);
            } else {
                visit_local_repositories(&path, callback)?;
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn exec_shell<P>(dir: P) -> Result<i32, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_owned());
    log::debug!("Exec {} in {}", shell, dir.as_ref().display());
    println!("chdir {}", dir.as_ref().display());

    use std::os::unix::process::CommandExt;
    let e = std::process::Command::new(shell).current_dir(dir).exec();
    Err(Box::new(e))
}

#[cfg(windows)]
fn exec_shell<P>(_dir: P) -> Result<i32, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    // TODO
    unimplemented!();
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_https_url() {
        assert_eq!(
            url::Url::parse("https://github.com/eagletmt/clg").unwrap(),
            super::parse_git_url("https://github.com/eagletmt/clg").unwrap()
        );
    }

    #[test]
    fn parse_ssh_url() {
        assert_eq!(
            url::Url::parse("ssh://git@github.com/eagletmt/clg").unwrap(),
            super::parse_git_url("ssh://git@github.com/eagletmt/clg").unwrap()
        );
    }

    #[test]
    fn parse_scp_like_ssh_url() {
        assert_eq!(
            url::Url::parse("ssh://git@github.com/eagletmt/clg").unwrap(),
            super::parse_git_url("git@github.com:eagletmt/clg").unwrap()
        );
    }

    fn tmp_config() -> super::super::Config {
        super::super::Config {
            root: std::path::PathBuf::from("/tmp"),
        }
    }

    #[test]
    fn destination_path_default() {
        assert_eq!(
            super::destination_path_for(
                &tmp_config(),
                &url::Url::parse("https://github.com/eagletmt/clg").unwrap(),
                None,
            )
            .unwrap(),
            std::path::PathBuf::from("/tmp/github.com/eagletmt/clg")
        );
    }

    #[test]
    fn destination_path_with_extension() {
        assert_eq!(
            super::destination_path_for(
                &tmp_config(),
                &url::Url::parse("https://github.com/eagletmt/clg.git").unwrap(),
                None,
            )
            .unwrap(),
            std::path::PathBuf::from("/tmp/github.com/eagletmt/clg")
        );
    }

    #[test]
    fn destination_path_with_name() {
        assert_eq!(
            super::destination_path_for(
                &tmp_config(),
                &url::Url::parse("https://github.com/eagletmt/clg.git").unwrap(),
                Some("clg2"),
            )
            .unwrap(),
            std::path::PathBuf::from("/tmp/github.com/eagletmt/clg2")
        );
    }
}
