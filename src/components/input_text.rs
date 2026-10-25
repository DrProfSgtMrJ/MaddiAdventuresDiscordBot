use poise::serenity_prelude::{
    CreateActionRow,
    CreateInputText,
    InputTextStyle,
};
use std::str::FromStr;

#[derive(Debug)]
pub enum InputText {
    PlayerName
}

impl InputText {
    pub fn title(&self) -> String {
        match self {
            InputText::PlayerName => "Player Name".to_string(),
        }
    }

    pub fn id(&self) -> String {
        match self {
            InputText::PlayerName => "player_name".to_string(),
        }
    }

    pub fn placeholder(&self) -> String {
        match self {
            InputText::PlayerName => "Enter your player name".to_string(),
        }
    }

    pub fn max_length(&self) -> u16 {
        match self {
            InputText::PlayerName => 32,
        }
    }

    pub fn required(&self) -> bool {
        match self {
            InputText::PlayerName => true,
        }
    }
}

impl FromStr for InputText {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "player_name" => Ok(InputText::PlayerName),
            _ => Err(()),
        }
    }
}

impl From<InputText> for CreateActionRow {
    fn from(input_text: InputText) -> Self {
        CreateActionRow::InputText(
            CreateInputText::new(InputTextStyle::Short, input_text.title(), input_text.id())
            .placeholder(input_text.placeholder())
            .max_length(input_text.max_length())
            .required(input_text.required())
        )
    }
}