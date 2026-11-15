use std::io::{self, Write};
use rpassword;

pub enum InputKind {
    Text,
    Password,
    Number,
    ServerURL,
}

pub fn prompt_user(msg: &str, kind: InputKind) -> String {
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
                input = format!("https://{}", input);
            }
            if input.starts_with("http://") {
                println!("Sending request over `http`, login can be intercepted! Use `https` for a secure connection")
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
