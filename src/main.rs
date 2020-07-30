extern crate exitcode;

use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(ping)]
struct General;

use std::env;
use std::process;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No Token provided as argument 1");
        process::exit(exitcode::DATAERR);
    }
    let token = &args[1];
    let mut client = Client::new(token, Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

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