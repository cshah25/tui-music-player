use std::sync::mpsc::Sender;
use std::thread;

use crate::action::Action;
use crate::state::{AppState, AuthState, Loadable};
use crate::{auth, config, subsonic};

pub fn update(state: &mut AppState, action: Action, tx: &Sender<Action>) {
    match action {
        Action::Login { uname, password, server } => {
            state.auth = AuthState::LoggingIn;
            let tx = tx.clone();
            thread::spawn(move || {
                let result = match auth::login(&uname, &password, &server) {
                    Ok(cfg) => Action::LoginSucceeded(cfg),
                    Err(msg) => Action::LoginFailed(msg),
                };
                let _ = tx.send(result);
            });
        }
        Action::LoginSucceeded(cfg) => match config::save(&cfg) {
            Ok(()) => state.auth = AuthState::LoggedIn(cfg),
            Err(e) => state.auth = AuthState::Failed(format!("Could not save login: {e}")),
        },
        Action::LoginFailed(msg) => state.auth = AuthState::Failed(msg),
        Action::LoadPlaylists => {
            if let AuthState::LoggedIn(cfg) = &state.auth {
                state.playlists = Loadable::Loading;
                let cfg = cfg.clone();
                let tx = tx.clone();
                thread::spawn(move || {
                    let result = match subsonic::get_playlists(&cfg) {
                        Ok(list) => Action::PlaylistsLoaded(list),
                        Err(msg) => Action::PlaylistsFailed(msg)
                    };
                    let _ = tx.send(result);
                });
            }
        }
        Action::PlaylistsLoaded(list) => state.playlists = Loadable::Loaded(list),
        Action::PlaylistsFailed(msg) => state.playlists = Loadable::Failed(msg)
    }
}
