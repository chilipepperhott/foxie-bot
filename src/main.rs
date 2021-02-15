mod images;
mod dirty_talk;
mod roastme;

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
use std::env;
use images::IMAGES_GROUP;
use dirty_talk::DIRTYTALK_GROUP;
use roastme::ROASTME_GROUP;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let token = args[1].clone();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&IMAGES_GROUP)
        .group(&DIRTYTALK_GROUP)
        .group(&ROASTME_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}