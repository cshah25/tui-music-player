use random_string::generate;
use reqwest::blocking::Client;

use crate::config::Config;

const SALT_CHARSET: &str = "1234567890qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

pub fn login(uname: &str, password: &str, server: &str) -> Result<Config, String> {
    let salt = generate(20, SALT_CHARSET);
    let token = format!("{:x}", md5::compute(format!("{password}{salt}")));

    let config = Config {
        uname: uname.to_string(),
        token,
        salt,
        server_url: server.to_string(),
    };

    let body = Client::new()
        .post(format!("{}/rest/ping.view", config.server_url))
        .form(&[
            ("u", config.uname.as_str()),
            ("t", config.token.as_str()),
            ("s", config.salt.as_str()),
            ("v", "1.16.1"),
            ("c", "tui-music-player"),
            ("f", "json"),
        ])
        .send()
        .map_err(|e| format!("Could not reach server: {e}"))?
        .text()
        .map_err(|e| format!("Bad response from server: {e}"))?;

    if body.contains("\"status\":\"ok\"") {
        Ok(config)
    } else {
        Err("Wrong username or password".to_string())
    }
}
