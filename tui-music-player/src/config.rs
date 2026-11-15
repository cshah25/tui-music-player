use std::io::{Write};
use std::path::PathBuf;
use random_string::generate;
use md5;
use std::fs;
use std::fs::OpenOptions;
use std::os::unix::fs::OpenOptionsExt;
use crate::input::{prompt_user, InputKind};

pub struct Config {
    pub uname: String,
    pub token: String,
    pub salt: String,
    pub server_url: String,
}

fn gen_salt(mut charset: Option<String>) -> String {
    charset.get_or_insert("1234567890!@#$%^&*()qwertyuiopasdfghjklzxcvbnm-=_+[]{};:',.<>/?".to_string());
    return generate(20, charset.unwrap());
}

fn read_config(path: &PathBuf) -> Config {
    let contents = fs::read_to_string(path).expect("failed to read config file");

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
                _ => {} // ignore unknown lines
            }
        }
    }

    config
}

pub fn rw_config() -> Config {
    let home = std::env::var("HOME").expect("HOME not set");
    let config_dir = PathBuf::from(home).join(".config/tui-music-player");
    fs::create_dir_all(&config_dir).expect("failed to create config dir");
    let config_path = config_dir.join("config");

    if config_path.exists() {
        return read_config(&config_path);
    }

    // No config yet: ask the user and save it
    let uname = prompt_user("Username: ", InputKind::Text);
    let password = prompt_user("Password: ", InputKind::Password);
    let server_url = prompt_user("Server URL (Defaults to `https://`): ", InputKind::ServerURL);

    let salt = gen_salt(None);
    let token = format!("{:x}", md5::compute(format!("{}{}", password, salt)));

    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600) // owner read/write only
        .open(&config_path)
        .expect("failed to create config file");
    writeln!(f, "uname:{}", uname).expect("failed to write config");
    writeln!(f, "token:{}", token).expect("failed to write config");
    writeln!(f, "salt:{}", salt).expect("failed to write config");
    writeln!(f, "server:{}", server_url).expect("failed to write config");

    Config { uname, token, salt, server_url }
}
