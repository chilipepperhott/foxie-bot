use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    CommandResult,
    macros::{
        command,
        group,
    },
    StandardFramework,
};
use serenity::model::channel::Message;

const ROASTS: &[&str] = &[
    "There may be no 'I' in team, but there is definitely a 'U' in cunt",
    "they're's no 'u' in teem \nBut there is one in retaurd:smirk::100::100::100::100::100::ok_hand::ok_hand::ok_hand::ok_hand::ok_hand:",
    "Anyone willing to fuck you is just too lazy to masturbate",
    "Your dick is sooo small when you put it inside a girl her immune system fights it.",
    "I would rather walk a thousand miles with a wet sock on my right foot, a pebble in my left shoe and my underwear crept up just beyond the stage it's acceptable, than to spend one more second listening to that death rattle voice of yours",
    "Maybe your dick wouldn't be so small if two thirds of it weren't your personality.",
    "I honestly think it would have been better if your mom had just done anal.",
    "Does your asshole get jealous of all the shit that comes out of your mouth?",
    "I'm not saying that you're a slut, but you're spit would be accepted at a sperm bank.",
    "You're not a prick you're the whole fucking cactus",
    "You’re like Rapunzel except instead of letting down your hair, you let everyone down in your life",
    "People like you are the reason they put instructions on shampoo bottles.",
    "You’re the human equivalent of a participation award",
    "Even Rick Astley would give you up",
    "If being sexy was a crime you'd be a law-abiding citizen.",
    "If intellect had mass, you'd have to be tied down so you don't float away.",
    "You’re the physical embodiment of hotdog water",
    "i hope you find earbuds with only one side working",
    "I hope an American mistakes you for a school",
    "Where's pest control when you need'em?"];

#[group]
#[commands(roast)]
struct RoastMe;

#[command("fuck you foxie")]
async fn roast(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, ROASTS[rand::random::<usize>() % ROASTS.len()].clone()).await?;

    Ok(())
}