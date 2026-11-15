use crate::config::Config;

pub enum AuthState {
    LoggedOut,
    LoggingIn,
    LoggedIn(Config),
    Failed(String),
}

pub struct AppState {
    pub auth: AuthState,
}