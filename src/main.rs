extern crate exitcode;

use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, gateway::Ready};
use serenity::prelude::{Context, EventHandler};

#[group]
#[commands(ping)]
struct General;

use std::env;
use std::process;

struct Handler;

impl EventHandler for Handler {
    // this only runs if there is no client.with_framework........
    fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "!ping" => send_message(&ctx, msg, "Second pong"),
            "!shard" => send_message(&ctx, msg, format!("Shard {}", ctx.shard_id).as_str()),
            "!dm" => try_send_dm(&ctx, msg, "DM test!"),
            _ => (),
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} connected", ready.user.name);
    }
}

fn try_send_dm(ctx: &Context, msg: Message, text: &str) {
    let dm = msg.author.dm(ctx, |m| {
        m.content(text);

        return m;
    });
    if let Err(why) = dm {
        println!("DM Err: {:?}", why);
    }
}

fn send_message(ctx: &Context, msg: Message, text: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, text) {
        println!("Err msg: {:?}", why);
    }
}

fn main() {
    // Login with a bot token from the environment
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No Token provided as argument 1");
        process::exit(exitcode::DATAERR);
    }
    let token = &args[1];
    let mut client = Client::new(token, Handler).expect("Error creating client");
    //client.with_framework(StandardFramework::new()
    //    .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
    //    .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}
