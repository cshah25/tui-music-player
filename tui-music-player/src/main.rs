use std::io::{self, Write};
use random_string::generate;
use rpassword;
use md5;
use reqwest::blocking::Client;

enum InputKind {
    Text,
    Password,
    Number,
    ServerURL,
}

fn gen_salt(mut charset: Option<String>) -> String {
    charset.get_or_insert("1234567890!@#$%^&*()qwertyuiopasdfghjklzxcvbnm-=_+[]{};:',.<>/?".to_string());
    return generate(20, charset.unwrap());
}

fn prompt_user(msg: &str, kind: InputKind) -> String {
    loop {
        let mut input = match kind {
            InputKind::Password => rpassword::prompt_password(msg).unwrap(),
            _ => {
                print!("{}", msg);
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                buf.trim().to_string()
            }
        };

        if input.chars().last().is_none() {
            println!("Please enter a valid server url.");
            continue;
        }

        if let InputKind::ServerURL = kind {
            input = input.trim_end_matches('/').to_string();
            if !(input.starts_with("http://") || input.starts_with("https://")) {
                input = format!("http://{}", input);
            }
        }

        if let InputKind::Number = kind {
            if input.parse::<i64>().is_err() {
                println!("Please enter a valid number.");
                continue;
            }
        }

        return input;
    }
}

fn auth() {
    let uname = prompt_user("Username: ", InputKind::Text);
    let password = prompt_user("Password: ", InputKind::Password);
    let server_url = prompt_user("Server URL (Defaults to `http://`): ", InputKind::ServerURL);
    // salt
    let salt = gen_salt(None);
    let token = format!("{:x}", md5::compute(format!("{}{}", password, salt)));

    // sending the request
    let client = Client::new();
    let res = client
        .post(server_url + "/rest/ping.view")
            .form(& [
                ("u", uname),
                ("t", token),
                ("s", salt),
                ("v", "1.16.1".to_string()),
                ("c", "myapp".to_string()),
                ("f", "json".to_string()),
            ]).send().expect("request failed");
    println!("Status: {}", res.status());
    println!("{}", res.text().unwrap());
}

fn main() {
    auth();
}
