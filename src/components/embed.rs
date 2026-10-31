use poise::serenity_prelude::{
    CreateEmbed,
    CreateEmbedFooter,
    CreateAttachment
};
use strum::IntoEnumIterator;

use crate::commands::Error;
use crate::models::class::Class;
use crate::models::stats::BaseStats;

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
static HEALTH_LABEL: &str = "Health";
static STRENGTH_LABEL: &str = "Strength";
static DEXTERITY_LABEL: &str = "Dexterity";
static INTELLIGENCE_LABEL: &str = "Intelligence";

#[derive(Debug, Clone)]
pub struct ClassEmbed {
    pub index: usize,
    pub class: Class,
    pub attachment: CreateAttachment
}

impl ClassEmbed {
    pub async fn new(index: usize, class: Class) -> Result<Self, Error> {
        let attachment = CreateAttachment::path(format!("{}/{}.jpeg", ASSETS_DIR, class)).await?;
        Ok(Self { index, class, attachment })
    }

    pub async fn create_embed(&self) -> Result<CreateEmbed, Error> {
        let s = self.class.base_stats();

        let attachment = &self.attachment;
        Ok(
            CreateEmbed::default()
                .title(format!("{:?}", self.class))
                .footer(CreateEmbedFooter::new(format!("Class {} of {}", self.index + 1, Class::iter().count())))
                .field(HEALTH_LABEL, s.health.to_string(), true)
                .field(STRENGTH_LABEL, s.strength.to_string(), true)
                .field(DEXTERITY_LABEL, s.dexterity.to_string(), true)
                .field(INTELLIGENCE_LABEL, s.intelligence.to_string(), true)
                .image(format!("attachment://{}", attachment.filename))
        )
    }
}