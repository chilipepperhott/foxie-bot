mod fun;
mod math;

use fun::FUN_GROUP;
use math::MATH_GROUP;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use std::collections::HashSet;
use std::env;

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
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let token = args[1].clone();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&MATH_GROUP)
        .group(&FUN_GROUP)
        .help(&HELP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
