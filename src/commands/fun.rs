extern crate hyper;
extern crate serde_json;

use self::hyper::client::Client as c;
use self::serde_json::Value;
use std::io::Read;
use serenity::client::Context;
use serenity::model::Message;

#[allow(unused_variables)]
pub fn cat_command(context: Context, _msg: Message, _args: Vec<String>) {
    let c = c::new();
    let mut content = String::new();
    let res = c.get("http://random.cat/meow").send().unwrap().read_to_string(&mut content);
    let data: Value = serde_json::from_str(&content).unwrap();
    let _ = context.say(&format!("{}",
                                 data.as_object().unwrap().get("file").unwrap().as_str().unwrap()));
}
