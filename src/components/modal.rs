use poise::serenity_prelude::CreateModal;
use std::str::FromStr;

#[derive(Debug)]
pub enum Modal {
    PlayerGetStarted,
}

impl Modal {
    pub fn title(&self) -> String {
        match self {
            Modal::PlayerGetStarted => "Get Started".to_string(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            Modal::PlayerGetStarted => "player_get_started".to_string(),
        }
    }
}

impl FromStr for Modal {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "player_get_started" => Ok(Modal::PlayerGetStarted),
            _ => Err(format!("Unknown modal id: {}", s)),
        }
    }
}

impl From<Modal> for CreateModal {
    fn from(modal: Modal) -> Self {
        CreateModal::new(modal.id(), modal.title())
    }
}