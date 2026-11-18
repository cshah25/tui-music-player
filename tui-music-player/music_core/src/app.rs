use std::sync::mpsc::{self, Receiver, Sender};

use crate::action::Action;
use crate::config;
use crate::state::{AppState, AuthState, Loadable};
use crate::update::update;

pub struct App {
    pub state: AppState,
    tx: Sender<Action>,
    rx: Receiver<Action>,
}

impl App {
    pub fn new() -> App {
        let (tx, rx) = mpsc::channel();
        let auth = match config::load() {
            Some(cfg) => AuthState::LoggedIn(cfg),
            None => AuthState::LoggedOut,
        };
        App { 
            state: AppState { 
                auth, 
                playlists: Loadable::NotLoaded, 
                artist_index: Loadable::NotLoaded, 
                genres: Loadable::NotLoaded,
                artist: Loadable::NotLoaded,
            },
            tx,
            rx
        }
    }

    pub fn dispatch(&mut self, action: Action) {
        update(&mut self.state, action, &self.tx);
    }

    pub fn process_background(&mut self) {
        while let Ok(action) = self.rx.try_recv() {
            update(&mut self.state, action, &self.tx);
        }
    }
}
