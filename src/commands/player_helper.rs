use poise::serenity_prelude::{
    CreateModal,
    CreateActionRow,
    CreateInteractionResponse,
};
use super::{Context, Error};

use crate::components::{modal::Modal, input_text::InputText};

pub async fn player_get_started_name_input(ctx: &Context<'_>) -> Result<(), Error> {
    let modal: CreateModal = Modal::PlayerGetStarted.into();
    let character_name_input: CreateActionRow = InputText::PlayerName.into();

    if let poise::Context::Application(app_ctx) = ctx {
        app_ctx
            .interaction
            .create_response(
                &ctx.serenity_context().http,
                CreateInteractionResponse::Modal(modal.components(vec![
                    character_name_input,
                ]))
            )
            .await?;
    } else {
        ctx.reply("This command only works as a slash command. Please use /get_started")
            .await?;
    }

    Ok(())
}