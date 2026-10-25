use std::{fmt::Display, str::FromStr};
use strum::{EnumIter};

// Defines the different character classes in the game
#[derive(Debug, EnumIter)]
pub enum Class {
    Warrior,
    Mage,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Class::Warrior => write!(f, "Warrior"),
            Class::Mage => write!(f, "Mage"),
        }
    }
}

impl FromStr for Class {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "warrior" => Ok(Class::Warrior),
            "mage" => Ok(Class::Mage),
            _ => Err(format!("Unknown class: {}", s)),
        }
    }
}
