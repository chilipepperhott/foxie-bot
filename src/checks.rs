use serenity::client::Context;
use serenity::framework::standard::{macros::check, Reason};
use serenity::model::prelude::Message;

const OWNERNAME: &str = "ChiliPepperHott#4147";

#[check("Owns bot")]
async fn is_bot_owner(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    if format!("{}#{}", msg.author.name, msg.author.discriminator) != OWNERNAME {
        return Err(Reason::UserAndLog {
            user: msg.author.name.to_owned(),
            log: "You are not the bot owner".to_string(),
        });
    }
    Ok(())
}
