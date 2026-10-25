/// Defines the player model in the game
use super::stats::{Stats, BaseStats};
use super::class::Class;

pub struct Player {
    pub name: String,
    pub class: Option<Class>,
    pub stats: Stats,
    pub level: u32,
}

impl Player {
    pub fn create(name: &str, class: Option<Class>) -> Self {
        let base_stats = match &class {
            Some(c) => c.base_stats(),
            None => Stats::default(),
        };

        Self {
            name: name.to_string(),
            class,
            stats: base_stats,
            level: 1,
        }
    }

    pub fn set_class(&mut self, class: Option<Class>) {
        self.class = class;
        self.stats = match &self.class {
            Some(c) => c.base_stats(),
            None => Stats::default(),
        };
    }

    pub fn level_up(&mut self) {
        self.level += 1;
    }
}