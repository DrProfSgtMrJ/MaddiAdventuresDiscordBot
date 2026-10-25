// Admin commands for the bot
use super::{Context, Error, Data};

use poise::serenity_prelude as serenity;

use crate::models::user::User;

pub fn register_admin_commands() -> Vec<poise::Command<Data, Error>> {
    vec![
        // Add your admin commands here
        add_user(),
        remove_user(),
        list_users()
    ]
}


#[poise::command(slash_command, owners_only = true)]
pub async fn add_user(
    ctx: Context<'_>,
    #[description = "The member to add"]
    member: serenity::Member
) -> Result<(), Error> {
    // Add logic to add the user to the system here
    let user = User {
        id: member.user.id.get(),
        display_name: member.display_name().to_string()
    };

    // TODO: Add to database
    Ok(())
}

#[poise::command(slash_command, owners_only = true)]
pub async fn remove_user(
    ctx: Context<'_>,
    #[description = "The member to remove"]
    member: serenity::Member
) -> Result<(), Error> {
    // Add logic to remove the user from the system here
    // TODO: Remove from database
    Ok(())
}

#[poise::command(slash_command, owners_only = true)]
pub async fn list_users(
    ctx: Context<'_>,
) -> Result<(), Error> {
    // Add logic to list users from the system here
    // TODO: Fetch from database
    Ok(())
}

