use std::{borrow::Cow, mem, sync::Arc};

use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    http::AttachmentType,
    model::{channel::Message, id::UserId},
    prelude::TypeMapKey,
    utils::Color,
};

use tokio::sync::Mutex;

use rand::{thread_rng, Rng};

const KISSES: [&[u8]; 6] = [
    include_bytes!("../gifs/kiss/1.gif"),
    include_bytes!("../gifs/kiss/2.gif"),
    include_bytes!("../gifs/kiss/3.gif"),
    include_bytes!("../gifs/kiss/4.gif"),
    include_bytes!("../gifs/kiss/5.gif"),
    include_bytes!("../gifs/kiss/6.gif"),
];

const HUGS: [&[u8]; 8] = [
    include_bytes!("../gifs/hug/1.gif"),
    include_bytes!("../gifs/hug/2.gif"),
    include_bytes!("../gifs/hug/3.gif"),
    include_bytes!("../gifs/hug/4.gif"),
    include_bytes!("../gifs/hug/5.gif"),
    include_bytes!("../gifs/hug/6.gif"),
    include_bytes!("../gifs/hug/7.gif"),
    include_bytes!("../gifs/hug/8.gif"),
];

const CUDDLES: [&[u8]; 10] = [
    include_bytes!("../gifs/cuddle/1.gif"),
    include_bytes!("../gifs/cuddle/2.gif"),
    include_bytes!("../gifs/cuddle/3.gif"),
    include_bytes!("../gifs/cuddle/4.gif"),
    include_bytes!("../gifs/cuddle/5.gif"),
    include_bytes!("../gifs/cuddle/6.gif"),
    include_bytes!("../gifs/cuddle/7.gif"),
    include_bytes!("../gifs/cuddle/8.gif"),
    include_bytes!("../gifs/cuddle/9.gif"),
    include_bytes!("../gifs/cuddle/10.gif"),
];

const BULLYS: [&[u8]; 5] = [
    include_bytes!("../gifs/bully/1.gif"),
    include_bytes!("../gifs/bully/2.gif"),
    include_bytes!("../gifs/bully/3.gif"),
    include_bytes!("../gifs/bully/4.gif"),
    include_bytes!("../gifs/bully/5.gif"),
];

const PATS: [&[u8]; 8] = [
    include_bytes!("../gifs/pat/1.gif"),
    include_bytes!("../gifs/pat/2.gif"),
    include_bytes!("../gifs/pat/3.gif"),
    include_bytes!("../gifs/pat/4.gif"),
    include_bytes!("../gifs/pat/5.gif"),
    include_bytes!("../gifs/pat/6.gif"),
    include_bytes!("../gifs/pat/7.gif"),
    include_bytes!("../gifs/pat/8.gif"),
];

const LICKS: [&[u8]; 7] = [
    include_bytes!("../gifs/lick/1.gif"),
    include_bytes!("../gifs/lick/2.gif"),
    include_bytes!("../gifs/lick/3.gif"),
    include_bytes!("../gifs/lick/4.gif"),
    include_bytes!("../gifs/lick/5.gif"),
    include_bytes!("../gifs/lick/6.gif"),
    include_bytes!("../gifs/lick/7.gif"),
];

struct LastKiss;

impl TypeMapKey for LastKiss {
    type Value = Arc<Mutex<usize>>;
}

struct LastHug;

impl TypeMapKey for LastHug {
    type Value = Arc<Mutex<usize>>;
}

struct LastCuddle;

impl TypeMapKey for LastCuddle {
    type Value = Arc<Mutex<usize>>;
}

struct LastBully;

impl TypeMapKey for LastBully {
    type Value = Arc<Mutex<usize>>;
}

struct LastPat;

impl TypeMapKey for LastPat {
    type Value = Arc<Mutex<usize>>;
}
struct LastLick;

impl TypeMapKey for LastLick {
    type Value = Arc<Mutex<usize>>;
}

/// Actions a person can perform on another
#[group]
#[commands(kiss, hug, cuddle, bully, pat, lick)]
struct Actions;

/// Kiss someone
#[command]
async fn kiss(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut last_index = {
        let mut data = ctx.data.write().await;

        if !data.contains_key::<LastKiss>() {
            data.insert::<LastKiss>(Arc::new(Mutex::new(0)))
        }

        let clone = data.get::<LastKiss>().unwrap().clone();
        let guard = clone.lock().await;
        *guard
    };

    send_gif(ctx, msg, args, &KISSES, &mut last_index, "kisses").await?;

    {
        let data = ctx.data.read().await;

        let clone = data.get::<LastKiss>().unwrap().clone();
        let mut guard = clone.lock().await;
        *guard = last_index;
    }

    Ok(())
}

/// Hug someone
#[command]
async fn hug(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut last_index = {
        let mut data = ctx.data.write().await;

        if !data.contains_key::<LastHug>() {
            data.insert::<LastHug>(Arc::new(Mutex::new(0)))
        }

        let clone = data.get::<LastHug>().unwrap().clone();
        let guard = clone.lock().await;
        *guard
    };

    send_gif(ctx, msg, args, &HUGS, &mut last_index, "hugs").await?;

    {
        let data = ctx.data.read().await;

        let clone = data.get::<LastHug>().unwrap().clone();
        let mut guard = clone.lock().await;
        *guard = last_index;
    }

    Ok(())
}

/// Cuddle someone
#[command]
async fn cuddle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut last_index = {
        let mut data = ctx.data.write().await;

        if !data.contains_key::<LastCuddle>() {
            data.insert::<LastCuddle>(Arc::new(Mutex::new(0)))
        }

        let clone = data.get::<LastCuddle>().unwrap().clone();
        let guard = clone.lock().await;
        *guard
    };

    send_gif(ctx, msg, args, &CUDDLES, &mut last_index, "cuddles").await?;

    {
        let data = ctx.data.read().await;

        let clone = data.get::<LastCuddle>().unwrap().clone();
        let mut guard = clone.lock().await;
        *guard = last_index;
    }

    Ok(())
}

/// Bully someone
#[command]
async fn bully(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut last_index = {
        let mut data = ctx.data.write().await;

        if !data.contains_key::<LastBully>() {
            data.insert::<LastBully>(Arc::new(Mutex::new(0)))
        }

        let clone = data.get::<LastBully>().unwrap().clone();
        let guard = clone.lock().await;
        *guard
    };

    send_gif(ctx, msg, args, &BULLYS, &mut last_index, "bullies").await?;

    {
        let data = ctx.data.read().await;

        let clone = data.get::<LastBully>().unwrap().clone();
        let mut guard = clone.lock().await;
        *guard = last_index;
    }

    Ok(())
}

/// Pat someone
#[command]
async fn pat(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut last_index = {
        let mut data = ctx.data.write().await;

        if !data.contains_key::<LastPat>() {
            data.insert::<LastPat>(Arc::new(Mutex::new(0)))
        }

        let clone = data.get::<LastPat>().unwrap().clone();
        let guard = clone.lock().await;
        *guard
    };

    send_gif(ctx, msg, args, &PATS, &mut last_index, "pats").await?;

    {
        let data = ctx.data.read().await;

        let clone = data.get::<LastPat>().unwrap().clone();
        let mut guard = clone.lock().await;
        *guard = last_index;
    }

    Ok(())
}

/// Lick someone
#[command]
async fn lick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let mut last_index = {
        let mut data = ctx.data.write().await;

        if !data.contains_key::<LastLick>() {
            data.insert::<LastLick>(Arc::new(Mutex::new(0)))
        }

        let clone = data.get::<LastLick>().unwrap().clone();
        let guard = clone.lock().await;
        *guard
    };

    send_gif(ctx, msg, args, &LICKS, &mut last_index, "licks").await?;

    {
        let data = ctx.data.read().await;

        let clone = data.get::<LastLick>().unwrap().clone();
        let mut guard = clone.lock().await;
        *guard = last_index;
    }

    Ok(())
}

/// Sends a random gif from the possible_gifs. Last_index is used to make sure that the same gif isn't sent twice
async fn send_gif(
    ctx: &Context,
    msg: &Message,
    mut args: Args,
    possible_gifs: &[&[u8]],
    last_index: &mut usize,
    verb: &str,
) -> CommandResult {
    msg.channel_id.broadcast_typing(ctx).await?;

    if msg.guild_id.is_none() {
        msg.reply(ctx, "Sorry but this command does not work in DMs")
            .await?;
        return Ok(());
    }

    let image = {
        let mut rng = thread_rng();
        *last_index = {
            let mut index = rng.gen_range(0..possible_gifs.len());
            while *last_index == index {
                index = rng.gen_range(0..possible_gifs.len());
            }
            index
        };
        possible_gifs[*last_index].clone()
    };

    let title_text;

    let arg = args.rest();

    if arg.is_empty() {
        title_text = "the void".to_owned()
    } else if arg.to_lowercase() == msg.author.name.to_lowercase() {
        title_text = "themself".to_owned();
    } else if arg.starts_with("<@!") {
        let id = match arg[3..(arg.len() - 1)].parse() {
            Ok(id) => id,
            Err(err) => return Err(Box::new(err)),
        };

        match UserId(id).to_user(ctx).await {
            Ok(user) => title_text = user.name,
            Err(err) => return Err(Box::new(err)),
        }
    } else {
        let mut member = None;
        let members = msg.guild_id.unwrap().members(ctx, None, None).await?;

        for m in members {
            if arg.to_lowercase() == m.user.name.to_lowercase() {
                member = Some(m);
                break;
            }
        }

        title_text = match member {
            Some(m) => m.user.name,
            None => arg.to_owned(),
        }
    }

    println!("{}", title_text);

    msg.channel_id
        .send_message(ctx, |m| {
            m.reference_message(msg);

            m.add_file(AttachmentType::Bytes {
                data: Cow::from(image),
                filename: "img.gif".to_string(),
            });

            m.embed(|e| {
                e.title(format!("{} {} {}", msg.author.name, verb, title_text));
                e.color(Color::BLUE);
                e.image("attachment://img.gif")
            })
        })
        .await?;

    Ok(())
}
