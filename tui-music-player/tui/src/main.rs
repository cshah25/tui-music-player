mod input;

use std::thread;
use std::time::Duration;

use input::{prompt_user, InputKind};
use music_core::action::Action;
use music_core::state::AuthState;
use music_core::App;

fn main() {
    let mut app = App::new();

    loop {
        app.process_background();

        match &app.state.auth {
            AuthState::LoggedIn(config) => {
                println!("Logged in as {} on {}", config.uname, config.server_url);
                break;
            }
            AuthState::LoggedOut => ask_login(&mut app),
            AuthState::Failed(msg) => {
                println!("Login failed: {msg}");
                ask_login(&mut app);
            }
            AuthState::LoggingIn => thread::sleep(Duration::from_millis(50)),
        }
    }
}

fn ask_login(app: &mut App) {
    let uname = prompt_user("Username: ", InputKind::Text);
    let password = prompt_user("Password: ", InputKind::Password);
    let server = prompt_user("Server URL (Defaults to `https://`): ", InputKind::ServerURL);
    app.dispatch(Action::Login { uname, password, server });
}