extern crate serenity;
extern crate hyper;
extern crate serde_json;

mod commands;

use hyper::client::Client as c;
use serde_json::Value;
use std::io::Read;
use commands::info;
use serenity::client::Context;
use serenity::Client;
use serenity::model::Message;

fn main() {
    let mut client = Client::login_bot("token");
    client.on_message(|_context, message| {
        println!("Received message: {:?}", message.content);
    });

    client.on_ready(|_context, ready| {
        _context.set_game_name("With Senpai");
        println!("{} is connected!", ready.user.name);
    });

    client.with_framework(|f| {
        f.configure(|c| {
                c.on_mention(true)
                    .allow_whitespace(false)
                    .prefix("~")
            })
            .on("cat", cat_command)
            .on("info", info::member_info)
    });

    let _ = client.start();
}

fn cat_command(context: Context, _msg: Message, _args: Vec<String>) {
    let c = c::new();
    let mut res = c.get("http://random.cat/meow").send().unwrap();
    let mut content = String::new();
    res.read_to_string(&mut content);
    let data: Value = serde_json::from_str(&content).unwrap();
    let _ = context.say(&format!("{}",
                                 data.as_object().unwrap().get("file").unwrap().as_str().unwrap()));
}
