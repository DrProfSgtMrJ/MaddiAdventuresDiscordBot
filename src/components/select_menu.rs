use std::str::FromStr;

use poise::serenity_prelude::{
    CreateActionRow,
    CreateSelectMenu,
    CreateSelectMenuKind,
    CreateSelectMenuOption
};

use strum::IntoEnumIterator;

use crate::models::class::Class;

#[derive(Debug)]
pub enum SelectMenu {
    ClassSelect
}

impl SelectMenu {
    pub fn id(&self) -> String {
        match self {
            SelectMenu::ClassSelect => "class_select".to_string(),
        }
    }

    pub fn placeholder(&self) -> &str {
        match self {
            SelectMenu::ClassSelect => "Select your class",
        }
    }

    pub fn options(&self) -> Vec<CreateSelectMenuOption> {
        match self {
            SelectMenu::ClassSelect => Class::iter()
            .map(|class| {
                CreateSelectMenuOption::new(class.to_string(), class.to_string())
            })
            .collect()
        }
    }

    pub fn kind(&self) -> CreateSelectMenuKind {
        match self {
            SelectMenu::ClassSelect  => CreateSelectMenuKind::String {
                options: self.options(),
            },
        }
    }

    pub fn min_values(&self) -> u8 {
        match self {
            SelectMenu::ClassSelect  => 1,
        }
    }

    pub fn max_values(&self) -> u8 {
        match self {
            SelectMenu::ClassSelect  => 1,
        }
    }
}

impl From<SelectMenu> for CreateSelectMenu {
    fn from(select_menu: SelectMenu) -> Self {
        CreateSelectMenu::new(select_menu.id(), select_menu.kind())
        .placeholder(select_menu.placeholder())
        .min_values(select_menu.min_values())
        .max_values(select_menu.max_values())
    }
}

impl From<SelectMenu> for CreateActionRow {
    fn from(select_menu: SelectMenu) -> Self {
        CreateActionRow::SelectMenu(select_menu.into())
    }
}

impl FromStr for SelectMenu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(':').collect::<Vec<&str>>();
        let id = parts.first().ok_or("Invalid")?;
        match *id {
            "class_select" => Ok(SelectMenu::ClassSelect),
            _ => Err("Invalid select menu id".into()),
        }
    }
}