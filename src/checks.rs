use serenity::client::Context;
use serenity::framework::standard::{macros::check, Reason};
use serenity::model::prelude::Message;

const OWNERNAME: &str = "ChiliPepperHott#4147";
const LEE_NAME: &str = "ToxicGarbage#191";
const MICHELLE_NAME: &str = "Mochi#3133";

#[check("Owns bot")]
pub async fn is_bot_owner_or_guest(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    let name = format!("{}#{}", msg.author.name, msg.author.discriminator);
    if !(name == LEE_NAME || name == OWNERNAME || name == MICHELLE_NAME) {
        return Err(Reason::UserAndLog {
            user: msg.author.name.to_owned(),
            log: "You are not the bot owner".to_string(),
        });
    }
    Ok(())
}
