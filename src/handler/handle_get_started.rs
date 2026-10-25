use poise::serenity_prelude::{
    ActionRowComponent, CreateInteractionResponse, CreateInteractionResponseMessage, ModalInteraction, ModalInteractionCollector
};
use std::time::Duration;

use crate::components::{modal::Modal, input_text::InputText};
use crate::commands::{Context, Error};

pub async fn handle_get_started_name_input(
    ctx: &Context<'_>
) -> Result<String, Error> { 
    println!("Waiting for player to input their name...");
    let Some(modal_response) = ModalInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![Modal::PlayerGetStarted.id()])
        .timeout(Duration::from_secs(300))
        .await
    else  {
        return Err("Failed to collect modal response".into());
    };

    println!("custom_id: {}", modal_response.data.custom_id);
    // Awknowledge the modal response to avoid it timing out
    modal_response
        .create_response(
            ctx.serenity_context(),
            CreateInteractionResponse::Acknowledge
        )
        .await?;

    let components = &modal_response.data.components;
    let mut name = None;
    for row in components {
        for component in &row.components {
            if let ActionRowComponent::InputText(input) = component
                && input.custom_id == InputText::PlayerName.id()
            {
                name = Some(input.value.as_ref());
            } else {
                println!("Unexpected component: {:?}", component);
            }
        }
    }
    let name = name.ok_or("Failed to get name input")?.map_or_default(|s| s.to_string());

    Ok(name)
}