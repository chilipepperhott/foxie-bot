use std::collections::HashSet;
use std::env;
use std::fs::{File, OpenOptions};

use log::error;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    Args, CommandGroup, CommandResult, help_commands, HelpOptions, macros::help, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::id::UserId;
use simplelog::*;

use fun::FUN_GROUP;
use math::MATH_GROUP;

mod fun;
mod math;

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
    // Setup Logging
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), OpenOptions::new().write(true).create(true).open("log.txt").expect("Could not open log file")),
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Stdout),
    ]).expect("Could not set up logger");
    error!("test");

    let token;
    match env::var("FOXIE_TOKEN"){
        Ok(t) => token = t,
        Err(_) => {
            error!("Could not get bot token from env");
            return;
        }
    }

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
        error!("An error occurred while running the client: {:?}", why);
    }
}
