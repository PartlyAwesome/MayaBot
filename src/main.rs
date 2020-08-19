extern crate exitcode;

use serenity::{
    client::Client,
    framework::standard::{
        macros::{check, command, group},
        Args, CheckResult, CommandOptions, CommandResult, StandardFramework,
    },
    model::{
        channel::{Channel, Message},
        gateway::Ready,
    },
    prelude::{Context, EventHandler},
    utils::MessageBuilder,
};

#[group]
#[commands(mping, rping, build, shard, dm)]
struct General;

#[group]
#[only_in(guilds)]
#[checks(admin)]
#[commands(admin, slow)]
struct Admin;

use std::env;
use std::process;

// Event Handler

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        // handle standard messages
        println!("{}", msg.content);
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

// Utility Functions

fn try_send_dm(ctx: &Context, msg: &Message, content: impl std::fmt::Display) {
    let dm = msg.author.dm(ctx, |m| {
        m.content(content);

        return m;
    });
    if let Err(why) = dm {
        println!("DM Err: {:?}", why);
    }
}

fn send_message(ctx: &Context, msg: &Message, content: impl std::fmt::Display) {
    if let Err(why) = msg.channel_id.say(&ctx.http, content) {
        println!("Err msg: {:?}", why);
    }
}

// Main

fn main() {
    // Login with a bot token from the environment
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No Token provided as argument 1");
        process::exit(exitcode::DATAERR);
    }
    let token = &args[1];
    let mut client = Client::new(token, Handler).expect("Error creating client");
    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP)
            .group(&ADMIN_GROUP),
    );

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

// Checks

#[check]
#[name = "admin"]
fn check_admin(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx) {
        if let Ok(perms) = member.permissions(ctx) {
            return perms.administrator().into();
        }
    }

    println!("Failed admin check");
    false.into()
}

// Commands

#[command]
fn mping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
fn rping(ctx: &mut Context, msg: &Message) -> CommandResult {
    send_message(ctx, msg, "Second pong");

    Ok(())
}

#[command]
fn build(ctx: &mut Context, msg: &Message) -> CommandResult {
    let mut mb = MessageBuilder::new();
    mb.push("User ")
        .mention(&msg.author)
        .push(" used !build; channel is ");
    match msg.channel(&ctx) {
        Some(channel) => mb.mention(&channel),
        None => mb.push("a channel"),
    };
    let content = mb.build();
    send_message(ctx, msg, content);
    Ok(())
}

#[command]
fn shard(ctx: &mut Context, msg: &Message) -> CommandResult {
    send_message(ctx, msg, format!("Shard {}", ctx.shard_id));
    Ok(())
}

#[command]
fn dm(ctx: &mut Context, msg: &Message) -> CommandResult {
    try_send_dm(ctx, msg, "DM test!");
    Ok(())
}

#[command]
fn admin(ctx: &mut Context, msg: &Message) -> CommandResult {
    send_message(ctx, msg, "Yes");
    Ok(())
}

#[command]
fn slow(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let content = if let Ok(rate) = args.single::<u64>() {
        if let Err(why) = msg.channel_id.edit(&ctx.http, |c| c.slow_mode_rate(rate)) {
            format!("Failed slow({}), reason: {:?}", rate, why)
        } else {
            format!("Success slow({})", rate)
        }
    } else {
        match msg.channel(&ctx) {
            Some(Channel::Guild(channel)) => format!(
                "Current slow({})",
                channel.read().slow_mode_rate.unwrap_or(0)
            ),
            _ => "Error finding channel".to_string(),
        }
    };

    send_message(ctx, msg, content);

    Ok(())
}
