use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;

#[group]
#[commands(math)]
struct Math;

#[command]
async fn math(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;

    match meval::eval_str(args.rest()) {
        Ok(n) => msg.reply(ctx, format!("{}", n)).await?,
        Err(_) => msg.reply(ctx, "I don't think I can calculate that").await?,
    };

    Ok(())
}
