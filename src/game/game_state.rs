use crate::config::Config;
use std::sync::{Arc, RwLock};

pub struct GameState {
    pub config: Arc<RwLock<Config>>,
}

impl GameState {
    pub fn new(config: Config) -> Self {
        GameState {
            config: Arc::new(RwLock::new(config)),
        }
    }

}
