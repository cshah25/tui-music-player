mod input;

use std::thread;
use std::time::Duration;

use input::{prompt_user, InputKind};
use music_core::action::Action;
use music_core::state::{ AuthState, Loadable };
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

    app.dispatch(Action::LoadPlaylists);

    loop {
        app.process_background();

        match &app.state.playlists {
            Loadable::Loaded(playlists) => {
                for p in playlists {
                    println!("{} ({} songs)", p.name, p.song_count);
                }
                break;
            }
            Loadable::Failed(msg) => {
                println!("Could not load playlists: {msg}");
                break;
            }
            _ => thread::sleep(Duration::from_millis(50)),
        }
    }

    app.dispatch(Action::LoadArtists);

    loop {
        app.process_background();
        match &app.state.artist_index {
            Loadable::Loaded(artist_index) => {
                println!("Artist index len: {}", artist_index.len());
                for a in artist_index {
                    println!("------------Index {}------------", a.name);
                    for artist in &a.artists {
                        println!("-----------------Artist name: {}-----------------", artist.name);
                        println!("Artist id: {}", artist.id);
                        println!("Artist Album Count: {}", artist.album_count);
                        if artist.cover_art.is_some() {
                            println!("Artist Cover Art: {}", artist.cover_art.as_ref().unwrap().to_string());
                        } else {
                            println!("Artist Cover Art Does not exist");
                        }
                    }
                }
                break;
            }
            Loadable::Failed(msg) => {
                println!("Could not load artists: {msg}");
                break;
            }

            _ => thread::sleep(Duration::from_millis(50)),
        }
    }
}

fn ask_login(app: &mut App) {
    let uname = prompt_user("Username: ", InputKind::Text);
    let password = prompt_user("Password: ", InputKind::Password);
    let server = prompt_user("Server URL (Defaults to `https://`): ", InputKind::ServerURL);
    app.dispatch(Action::Login { uname, password, server });
}
