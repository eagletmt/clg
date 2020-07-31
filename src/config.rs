#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
    #[serde(default = "default_root")]
    pub root: std::path::PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: default_root(),
        }
    }
}

fn default_root() -> std::path::PathBuf {
    let mut root = dirs::home_dir().expect("Cannot get HOME directory");
    root.push(".clg");
    root
}

impl Config {
    pub fn load_from_file() -> Config {
        let mut path = dirs::home_dir().expect("Cannot get HOME directory");
        path.push(".clg.toml");
        if path.exists() {
            match std::fs::File::open(&path) {
                Ok(mut file) => {
                    use std::io::Read;
                    let mut buf = vec![];
                    match file.read_to_end(&mut buf) {
                        Ok(_) => {
                            let r: Result<Config, toml::de::Error> = toml::from_slice(&buf);
                            match r {
                                Ok(config) => config,
                                Err(e) => {
                                    log::error!("Failed to deserialize {}: {}", path.display(), e);
                                    Default::default()
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Failed to read {}: {}", path.display(), e);
                            Default::default()
                        }
                    }
                }
                Err(e) => {
                    log::error!("Failed to open {}: {}", path.display(), e);
                    Default::default()
                }
            }
        } else {
            log::debug!("No ~/.clg.toml");
            Default::default()
        }
    }
}
