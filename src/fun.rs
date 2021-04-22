use super::{checks::IS_BOT_OWNER_OR_GUEST_CHECK, reddit_helpers::*};
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use uwuifier::uwuify_str_sse;

const HOT_PHRASES: &[&str] = &[
    "I need you right now.",
    "Damn, you smell good enough to eat.",
    "I’m craving you.",
    "Fuck me",
    "Hey, hot stuff",
    "I want you",
    "Your body is made up of 70% water. . .and I'm thirsty.",
    "Are you a campfire? Because you're hot and I want s'more.",
    "Roses are red. Violets are fine. You be the 6. I'll be the 9.",
    "I love my bed but I’d rather be in yours.",
    "You know, if I were you, I’d have sex with me.",
    "You’re on my list of things to do tonight.",
    "Fuck me if I’m wrong but dinosaurs still exist, right?",
];

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
    "Where's pest control when you need'em?",
    "awww don’t hate me cause i’m beatiful, maybe if you got rid of the ol yee yee ass haircut you got, you’d get some bitches on ya dick, oh, better yet, maybe tanisha would call your dog ass if she ever stop fucking with that brain surgeon or lawyer she fuckin with,"];

#[group]
#[commands(hot, roast, meme, retard_meme, hentai, hotmen, animedude, uwuify, echo)]
struct Fun;

#[command]
/// Asks Foxie to get all hot and bothered with you
async fn hot(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;
    msg.reply(
        ctx,
        &(HOT_PHRASES[rand::random::<usize>() % HOT_PHRASES.len()]),
    )
    .await?;

    Ok(())
}

#[command("fuckyoufoxie")]
/// Asks Foxie to roast you
async fn roast(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;
    let roast = get_roast().await;

    msg.reply(ctx, roast).await?;

    Ok(())
}

/// Gets a random roast from Reddits r/insults subreddit, posted in the last month
async fn get_roast() -> String {
    // Get top 20 top posts from the last month
    let subreddit = Subreddit::new("insults");
    let posts;
    match subreddit
        .top(30, Some(FeedOption::new().period(TimePeriod::ThisMonth)))
        .await
    {
        Ok(p) => posts = p,
        Err(_) => {
            return get_preset_random();
        }
    }

    // Filter for ones without body text because they are usually better
    let mut tries: u8 = 30;
    let mut post = &posts.data.children[rand::random::<usize>() % posts.data.children.len()].data;
    while !post.selftext.is_empty() && tries > 0 {
        post = &posts.data.children[rand::random::<usize>() % posts.data.children.len()].data;
        tries -= 1;
    }

    if !post.selftext.is_empty() {
        return get_preset_random();
    }
    post.title.clone()
}

fn get_preset_random() -> String {
    (ROASTS[rand::random::<usize>() % ROASTS.len()]).to_string()
}

#[command]
/// Asks Foxie for a meme
async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;
    msg.reply(
        ctx,
        get_top_image_from_subreddit("memes", TimePeriod::ThisWeek).await,
    )
    .await?;

    Ok(())
}

#[command("imretarded")]
/// Asks foxie to get a meme from r/okbuddyretard
async fn retard_meme(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;
    msg.reply(
        ctx,
        get_top_image_from_subreddit("okbuddyretard", TimePeriod::ThisMonth).await,
    )
    .await?;

    Ok(())
}

#[command]
/// Asks foxie for a uwuified version of text
async fn uwuify(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let text = args.rest();

    msg.channel_id.broadcast_typing(ctx).await?;

    let _ = msg.delete(ctx).await;

    msg.channel_id
        .send_message(ctx, |m| m.content(uwuify_str_sse(text)))
        .await?;

    Ok(())
}

#[command]
#[checks(is_bot_owner_or_guest)]
/// Asks foxie from some that good-good
async fn hentai(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;
    msg.reply(
        ctx,
        get_top_image_from_subreddit("hentai", TimePeriod::ThisWeek).await,
    )
    .await?;

    Ok(())
}

#[command]
#[checks(is_bot_owner_or_guest)]
/// Asks foxie for some that good-good, but for girls
async fn hotmen(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;
    msg.reply(
        ctx,
        get_top_image_from_subreddit("nearlynudemen", TimePeriod::ThisWeek).await,
    )
    .await?;

    Ok(())
}

#[command]
#[checks(is_bot_owner_or_guest)]
/// Asks foxie for some that good-good, but for weeb girls
async fn animedude(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;
    msg.reply(
        ctx,
        get_top_image_from_subreddit("animeboys", TimePeriod::ThisWeek).await,
    )
        .await?;

    Ok(())
}

#[command]
#[checks(is_bot_owner_or_guest)]
/// Reposts content of message as foxie
async fn echo(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    let text = args.rest();

    msg.channel_id.broadcast_typing(ctx).await?;

    let _ = msg.delete(ctx).await;

    msg.channel_id
        .send_message(ctx, |m| m.content(text))
        .await?;

    Ok(())
}