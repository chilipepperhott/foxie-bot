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

const PHRASES: &[&str] = &[
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
    "Fuck me if I’m wrong but dinosaurs still exist, right?"];

#[group]
#[commands(hot)]
struct DirtyTalk;

#[command]
async fn hot(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, PHRASES[rand::random::<usize>() % PHRASES.len()].clone()).await?;

    Ok(())
}