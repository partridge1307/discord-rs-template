use poise::serenity_prelude::User;

use super::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Select specific user"] user: User,
) -> Result<(), Error> {
    let response = format!("{}'s created at {}", user.name, user.created_at());

    ctx.say(response).await?;

    Ok(())
}
