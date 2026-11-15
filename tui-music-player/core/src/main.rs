use reqwest::blocking::Client;

mod input;
mod config;

fn auth() {
    let config : config::Config = config::rw_config();
    // Sending the request
    let client = Client::new();
    let res = client
        .post(config.server_url + "/rest/ping.view")
            .form(&[
                ("u", config.uname),
                ("t", config.token),
                ("s", config.salt),
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
