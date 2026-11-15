use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;

pub struct Config {
    pub uname: String,
    pub token: String,
    pub salt: String,
    pub server_url: String,
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME not set");
    PathBuf::from(home).join(".config/tui-music-player/config")
}

pub fn load() -> Option<Config> {
    let contents = fs::read_to_string(config_path()).ok()?;

    let mut config = Config {
        uname: String::new(),
        token: String::new(),
        salt: String::new(),
        server_url: String::new(),
    };

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once(':') {
            match key {
                "uname" => config.uname = value.to_string(),
                "token" => config.token = value.to_string(),
                "salt" => config.salt = value.to_string(),
                "server" => config.server_url = value.to_string(),
                _ => {}
            }
        }
    }

    Some(config)
}

pub fn save(config: &Config) -> std::io::Result<()> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(&path)?;

    writeln!(f, "uname:{}", config.uname)?;
    writeln!(f, "token:{}", config.token)?;
    writeln!(f, "salt:{}", config.salt)?;
    writeln!(f, "server:{}", config.server_url)?;
    Ok(())
}