use std::collections::HashSet;
use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Activity;
use serenity::model::id::UserId;
use serenity::model::prelude::Ready;
use serenity::model::user::OnlineStatus;

use fun::FUN_GROUP;
use math::MATH_GROUP;

mod checks;
mod fun;
mod math;
mod reddit_helpers;

#[help]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _bot_status: Ready) {
        ctx.set_presence(
            Some(Activity::listening("your mom moaning")),
            OnlineStatus::Online,
        )
        .await;
    }
}

async fn run_bot() {
    let token;
    match env::var("FOXIE_TOKEN") {
        Ok(t) => token = t,
        Err(_) => {
            return;
        }
    }

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!").case_insensitivity(true))
        .group(&MATH_GROUP)
        .group(&FUN_GROUP)
        .help(&HELP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    client.start().await.unwrap();
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    run_bot().await;
}
