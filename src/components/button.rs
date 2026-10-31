use poise::serenity_prelude::{
    CreateButton,
    ButtonStyle,
};
use std::str::FromStr;

#[derive(Debug)]
pub enum ClassButton {
    Prev,
    Next,
    Select,
}

impl ClassButton{

    pub fn id(&self) -> String {
        match self {
            ClassButton::Prev => "class_prev".to_string(),
            ClassButton::Next => "class_next".to_string(),
            ClassButton::Select => "class_select".to_string(),
        }
    }

    pub fn style(&self) -> ButtonStyle {
        match self {
            ClassButton::Prev => ButtonStyle::Secondary,
            ClassButton::Next => ButtonStyle::Secondary,
            ClassButton::Select => ButtonStyle::Success,
        }
    }

    pub fn label(&self) -> String {
        match self {
            ClassButton::Prev => "◀".to_string(),
            ClassButton::Next => "▶".to_string(),
            ClassButton::Select => "Select".to_string(),
        }
    }
}

impl FromStr for ClassButton {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "class_prev" => Ok(ClassButton::Prev),
            "class_next" => Ok(ClassButton::Next),
            "class_select" => Ok(ClassButton::Select),
            _ => Err(format!("Unknown button id: {}", s)),
        }
    }
}

impl From<ClassButton> for CreateButton {
    fn from(button: ClassButton) -> Self {
        CreateButton::new(button.id())
            .label(button.label())
            .style(button.style())
    }
}