pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod admin;
pub mod player;
mod player_helper;

pub use admin::register_admin_commands;
pub use player::register_player_commands;
pub use player_helper::*;