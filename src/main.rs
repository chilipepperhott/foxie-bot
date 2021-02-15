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
use images::IMAGES_GROUP;
use dirty_talk::DIRTYTALK_GROUP;
use roastme::ROASTME_GROUP;

const TOKEN: &str = "ODEwNzUzMDEzMjE5MDY1OTE3.YCoOlQ.F5QcHoIZ2cBhwIeFpWCPDza7rgc";

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&IMAGES_GROUP)
        .group(&DIRTYTALK_GROUP)
        .group(&ROASTME_GROUP);

    let mut client = Client::builder(TOKEN)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}