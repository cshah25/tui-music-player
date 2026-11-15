// music_core/src/app.rs
use std::sync::mpsc::{self, Receiver, Sender};

use crate::action::Action;
use crate::config::Config;
use crate::state::AppState;
use crate::update::update;

pub struct App {
    pub state: AppState,      // frontends can read this to draw
    tx: Sender<Action>,       // services use this to send results back
    rx: Receiver<Action>,
}

impl App {
    pub fn new(config: Config) -> App {
        let (tx, rx) = mpsc::channel();
        App { state: AppState::new(config), tx, rx }
    }

    // The frontend calls this when the user does something
    pub fn dispatch(&mut self, action: Action) {
        update(&mut self.state, action, &self.tx);
    }

    // The frontend calls this every frame to apply finished background work
    pub fn process_background(&mut self) {
        while let Ok(action) = self.rx.try_recv() {
            update(&mut self.state, action, &self.tx);
        }
    }
}
