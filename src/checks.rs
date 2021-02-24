use serenity::client::Context;
use serenity::framework::standard::{macros::check, Reason};
use serenity::model::prelude::Message;

const OWNERNAME: &str = "ChiliPepperHott#4147";

#[check("Owns bot")]
pub async fn is_bot_owner_or_guest(ctx: &Context, msg: &Message) -> Result<(), Reason> {
    match msg.author.has_role(ctx, 810753616302309376, 810759629608058889).await{
        Ok(b) => {
            if b{
                return Ok(())
            }
        }
        Err(_) => ()
    }

    if format!("{}#{}", msg.author.name, msg.author.discriminator) != OWNERNAME {
        return Err(Reason::UserAndLog {
            user: msg.author.name.to_owned(),
            log: "You are not the bot owner".to_string(),
        });
    }
    Ok(())
}
