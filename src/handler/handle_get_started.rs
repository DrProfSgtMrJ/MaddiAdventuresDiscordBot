use poise::serenity_prelude::{
    ActionRowComponent,
    CreateActionRow,
    CreateAttachment,
    CreateEmbed,
    CreateEmbedFooter,
    CreateInteractionResponse,
    ModalInteraction,
    ModalInteractionCollector,
    CreateInteractionResponseMessage,
    EditInteractionResponse,
    ComponentInteractionCollector,
    EditAttachments
};
use strum::IntoEnumIterator;
use std::time::Duration;

use crate::components::{modal::Modal, input_text::InputText, button::ClassButton};
use crate::commands::{Context, Error};
use crate::models::class::Class;
use crate::models::stats::BaseStats;

pub async fn handle_get_started_name_input(
    ctx: &Context<'_>
) -> Result<(String, ModalInteraction), Error> { 
    println!("Waiting for player to input their name...");
    let Some(modal_response) = ModalInteractionCollector::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .custom_ids(vec![Modal::PlayerGetStarted.id()])
        .timeout(Duration::from_secs(300))
        .await
    else  {
        return Err("Failed to collect modal response".into());
    };

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

    Ok((name, modal_response))
}

const ASSETS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/assets");
async fn class_view(index: usize) -> Result<(CreateEmbed, Vec<CreateActionRow>, CreateAttachment), Error> {
    let class_vec: &Vec<Class> = &Class::iter().collect();
    let class = &class_vec[index];
    let s = class.base_stats();

    let attachment = CreateAttachment::path(format!("{}/{}.jpeg", ASSETS_DIR, class)).await?;
    let embed = CreateEmbed::default()
        .title(format!("{:?}", class))
        .footer(CreateEmbedFooter::new(format!("Class {} of {}", index + 1, class_vec.len())))
        .field("Health", s.health.to_string(), true)
        .field("Strength", s.strength.to_string(), true)
        .field("Dexterity", s.dexterity.to_string(), true)
        .field("Intelligence", s.intelligence.to_string(), true)
        .image(format!("attachment://{}", attachment.filename));

    let buttons = CreateActionRow::Buttons(vec![
        ClassButton::Prev.into(),
        ClassButton::Select.into(),
        ClassButton::Next.into(),
    ]);

    Ok((embed, vec![buttons], attachment))
}

pub async fn choose_class(
    ctx: &Context<'_>,
    modal: &ModalInteraction,
) -> Result<Option<Class>, Error> {
    let sctx = ctx.serenity_context();
    let class_vec: Vec<Class> = Class::iter().collect();
    let len = class_vec.len();
    let mut index = 0;

    // The one and only response to the modal
    let (embed, components, attachment) = class_view(index).await?;
    modal
        .create_response(
            sctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .components(components)
                    .add_file(attachment),
            ),
        )
        .await?;
    let message = modal.get_response(sctx).await?;

    loop {
        let Some(press) = ComponentInteractionCollector::new(sctx)
            .author_id(ctx.author().id)
            .message_id(message.id)
            .timeout(Duration::from_secs(120))
            .await
        else {
            // Timed out: remove the buttons
            modal.edit_response(sctx, EditInteractionResponse::new().components(vec![])).await?;
            return Ok(None);
        };

        // Ack immediately so Discord's 3-second window is never an issue
        press.create_response(sctx, CreateInteractionResponse::Acknowledge).await?;

        match press.data.custom_id.as_str() {
            "class_prev" => index = (index + len - 1) % len,
            "class_next" => index = (index + 1) % len,
            "class_select" => {
                let class = class_vec.get(index).unwrap();
                press
                    .edit_response(
                        sctx,
                        EditInteractionResponse::new()
                            .content(format!("You chose **{}**!", class))
                            .components(vec![]),
                    )
                    .await?;
                return Ok(Some(*class));
            }
            _ => continue,
        }

        let (embed, components, attachment) = class_view(index).await?;
        press
            .edit_response(
                sctx,
                EditInteractionResponse::new()
                    .embed(embed)
                    .components(components)
                    .attachments(EditAttachments::new().add(attachment)),
            )
            .await?;
    }
}