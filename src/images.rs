use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(monke)]
struct Images;

#[command]
async fn monke(ctx: &Context, msg: &Message) -> CommandResult{
    msg.reply(ctx, "https://tenor.com/view/monkiflip-flip-monkey-monki-anime-flip-gif-18708891").await?;

    Ok(())
}