use std::io::{self, Write, Read, stdout};
use random_string::generate;
use rpassword;
use md5;
use reqwest::blocking::Client;

fn auth() {
    // Username
    print!("Enter your username: ");
    io::stdout().flush().expect("failed to flush");
    let mut uname = String::new();
    io::stdin().read_line(&mut uname).expect("failed to readline");
    let uname = uname.trim();
    // Password
    let password = rpassword::prompt_password("Your password: ").unwrap();
    println!("");
    // salt
    let charset = "1234567890!@#$%^&*()qwertyuiopasdfghjklzxcvbnm-=_+[]{};:',.<>/?";
    let mut salt = String::new();
    salt = generate(20, charset);
    let token = format!("{:x}", md5::compute(format!("{}{}", password, salt)));

    // sending the request
    let mut data = String::new();
    let client = Client::new();
    let res = client
                .post("http://192.168.1.22:4533/rest/ping.view")
                .form(& [
                    ("u", uname),
                    ("t", token.as_str()),
                    ("s", salt.as_str()),
                    ("v", "1.16.1"),
                    ("c", "myapp"),
                    ("f", "json"),
                ])
                .send()
                .expect("request failed");
    println!("Status: {}", res.status());
    println!("{}", res.text().unwrap());
}

fn main() {
    auth();
}
