use poise::serenity_prelude as serenity;
use dotenvy::dotenv;

mod commands;
mod models;
mod components;
mod handler;


use commands::{Data, Error, register_admin_commands, register_player_commands};

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command {}: {:?}", ctx.command().name, error);
        }
        poise::FrameworkError::Setup { error, .. } => {
            println!("Error in setup: {:?}", error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error handling framework error: {:?}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("Starting bot...");

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::GUILDS | serenity::GatewayIntents::MESSAGE_CONTENT;

     let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                register_admin_commands().into_iter(),
                register_player_commands().into_iter()
            ].into_iter().flatten().collect(),
            on_error: |error| Box::pin(on_error(error)),
            event_handler: |_ctx, event| {
                Box::pin(async move {
                    if let serenity::FullEvent::InteractionCreate { interaction } = event {
                        println!("Interaction received: {:?}", interaction.kind());
                        if let Some(m) = interaction.as_modal_submit() {
                            println!("Modal submit received: {:?}", m.data.custom_id);
                        }
                    }
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();
      let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
