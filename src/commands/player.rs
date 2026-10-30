use poise::serenity_prelude::{
    CreateInteractionResponse,
    CreateInteractionResponseMessage
};

/// Player commands for the bot
use super::{Context, Error, Data};
//use crate::models::class::Class;
use super::player_helper::*;
use crate::handler::{
    handle_get_started_name_input,
    choose_class
};

pub fn register_player_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        // Add your player commands here
        get_started()
    ]
}

#[poise::command(slash_command)]
pub async fn get_started(
    ctx: Context<'_>
) -> Result<(), Error> {
    // Add logic to set up the player here
    // Creates player menu -> Allows user to select their class + name

    // TODO: Check if player is a registered user
    player_get_started_name_input(&ctx).await?;
    let (name, modal_response) = handle_get_started_name_input(&ctx).await?;
    println!("Player name: {}", name);
    let Some(class) = choose_class(&ctx, &modal_response).await? else {
        return Ok(());
    };
    println!("Player chose class: {}", class);

    Ok(())
}