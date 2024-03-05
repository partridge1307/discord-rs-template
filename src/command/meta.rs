use super::{Context, Error};

///Check latency command
#[poise::command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let latency = ctx.ping().await;

    ctx.say(format!("Pong! {}ms", latency.as_millis())).await?;

    Ok(())
}
