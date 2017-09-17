extern crate clap;
extern crate std;
extern crate url;

pub fn clone(matches: &clap::ArgMatches) -> Result<i32, super::Error> {
    let arg = matches.value_of("URL").unwrap();
    let uri = parse_git_url(arg)?;
    let path = destination_path_for(&uri)?;
    debug!("Clone {} to {}", uri, path.display());
    let mut child = std::process::Command::new("git")
        .arg("clone")
        .arg(arg)
        .arg(&path)
        .spawn()?;
    let status = child.wait()?;
    Ok(status.code().unwrap_or(1))
}

pub fn look(matches: &clap::ArgMatches) -> Result<i32, super::Error> {
    let repository = matches.value_of("REPOSITORY").unwrap();
    let root_dir = root_dir()?;
    let mut local_repos = vec![];
    visit_local_repositories(&root_dir, &mut |path| if path.ends_with(repository) {
        local_repos.push(path.to_path_buf());
    })?;
    if local_repos.is_empty() {
        eprintln!("No repository found matching {}", repository);
        Ok(1)
    } else if local_repos.len() == 1 {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_owned());
        debug!("Exec {} in {}", shell, local_repos[0].display());
        println!("chdir {}", local_repos[0].display());

        use std::os::unix::process::CommandExt;
        let e = std::process::Command::new(shell)
            .current_dir(&local_repos[0])
            .exec();
        Err(super::Error::from(e))
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

fn parse_git_url(u: &str) -> Result<url::Url, super::Error> {
    // https://git-scm.com/docs/git-push#_git_urls_a_id_urls_a
    match url::Url::parse(u) {
        Ok(uri) => {
            debug!("{} is absolute URI", u);
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
            debug!("{} is GitHub.com URI", u);
            // Map :user/:repo to https://github.com/:user/:repo
            Ok(url::Url::parse("https://github.com").unwrap().join(u)?)
        }
        Err(e) => Err(super::Error::from(e)),
    }
}

fn parse_scp_like_url(u: &str, colon_idx: usize) -> Result<url::Url, super::Error> {
    debug!("{} is scp-like URI", u);
    let user_and_host = &u[..colon_idx];
    let path = &u[colon_idx + 1..];
    Ok(url::Url::parse(
        &format!("ssh://{}/{}", user_and_host, path),
    )?)
}

fn destination_path_for(uri: &url::Url) -> Result<std::path::PathBuf, super::Error> {
    let mut pathbuf = root_dir()?;
    pathbuf.push(uri.host_str().unwrap());
    for c in std::path::PathBuf::from(uri.path()).components().skip(1) {
        pathbuf.push(c.as_os_str());
    }
    Ok(pathbuf)
}

fn root_dir() -> Result<std::path::PathBuf, super::Error> {
    match std::env::home_dir() {
        Some(mut pathbuf) => {
            pathbuf.push(".ghq"); // TODO: Make customizable
            Ok(pathbuf)
        }
        None => Err(super::Error::Custom("Cannot get HOME directory".to_owned())),
    }
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

#[cfg(test)]
mod test {
    #[test]
    fn parse_https_url() {
        assert_eq!(
            super::url::Url::parse("https://github.com/eagletmt/clg").unwrap(),
            super::parse_git_url("https://github.com/eagletmt/clg").unwrap()
        );
    }

    #[test]
    fn parse_ssh_url() {
        assert_eq!(
            super::url::Url::parse("ssh://git@github.com/eagletmt/clg").unwrap(),
            super::parse_git_url("ssh://git@github.com/eagletmt/clg").unwrap()
        );
    }

    #[test]
    fn parse_scp_like_ssh_url() {
        assert_eq!(
            super::url::Url::parse("ssh://git@github.com/eagletmt/clg").unwrap(),
            super::parse_git_url("git@github.com:eagletmt/clg").unwrap()
        );
    }
}
