use poise::serenity_prelude as serenity;
use std::env;

use crate::command::Data;

mod command;

#[tokio::main]
async fn main() {
    let framework_opts = poise::FrameworkOptions {
        commands: vec![command::meta::ping(), command::math::age()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            ..Default::default()
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {})
            })
        })
        .options(framework_opts)
        .build();

    let token = env::var("DISCORD_TOKEN").expect("Could not found discord token");

    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create client");

    if let Err(e) = client.start().await {
        panic!("Error while running: {:?}", e);
    };
}
