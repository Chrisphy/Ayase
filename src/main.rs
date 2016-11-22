extern crate serenity;
extern crate yaml_rust;

mod commands;

use std::fs::File;
use std::io::prelude::*;
use yaml_rust::yaml;
use commands::{fun, info};
use serenity::Client;

fn main() {
    let mut f = File::open("config.yaml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
    let doc = &docs[0];
    let mut client = Client::login_bot(&doc["token"][0].as_str().unwrap());
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
            .on("cat", fun::cat_command)
            .on("info", info::member_info)
            .on("server", info::guild_info)
    });

    let _ = client.start();
}
