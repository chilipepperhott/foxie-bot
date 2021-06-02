use std::time::Duration;

use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::model::id::ChannelId;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[group]
#[commands(kink, undo_kink)]
struct Kinks;

/// Adds a kink to the kink journal.
/// USAGE: !kink <name> <kink>
#[command]
async fn kink(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let name = match args.current(){
        Some(n) => n,
        None => {
            msg.reply(ctx, "You need to add at least a name to the kink").await?;
            return Ok(());
        }
    }.to_owned();
    args.advance();
    let kink = args.rest();

    println!("Adding kink: {}", kink);

    let channel_id = ChannelId(846765620377026561);
    let mut message = match get_last_id().await{
        Ok(id) => {
            channel_id.message(ctx, id).await?
        },
        Err(_) => {
            let message = channel_id.say(ctx, "**Kink Journal:**\n").await?;
            set_last_id(message.id.0).await?;
            message
        }
    };

    let previous_content = message.content.to_owned();

    if previous_content.len() + kink.len() > 2000{
        message = channel_id.say(ctx, "**Kink Journal:**").await?;
        set_last_id(message.id.0).await?;
    }

    message.edit(ctx, |m| {
        m.content(format!("{}\n -> {}: {}", previous_content, name, kink))
    }).await?;

    let response = msg.reply(ctx, "I added it, you sick fuck.").await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    msg.delete(ctx).await?;
    response.delete(ctx).await?;

    Ok(())
}

#[command("ukink")]
async fn undo_kink(ctx: &Context, msg: &Message) -> CommandResult{
    let channel_id = ChannelId(846765620377026561);

    let mut message = channel_id.message(ctx, get_last_id().await?).await?;

    let mut previous_content = message.content.to_owned();

    while match previous_content.pop(){
        Some(c) => c,
        None => {
            msg.reply(ctx, "Sorry hon, that doesn't work").await?;
            return Ok(());
        }
    } != '\n'{}

    message.edit(ctx, |m|{
        m.content(previous_content)
    }).await?;

    Ok(())
}

async fn get_last_id() -> Result<u64, std::io::Error>{
    let mut file = File::open("last_id").await?;
    file.read_u64().await
}

async fn set_last_id(id: u64) -> Result<(), std::io::Error>{
    let mut file = File::create("last_id").await?;
    file.write_u64(id).await
}