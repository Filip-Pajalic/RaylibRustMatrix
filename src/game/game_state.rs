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

    pub fn update_config(&self, new_config: Config) {
        let mut config = self.config.write().unwrap();
        *config = new_config;
    }
}
