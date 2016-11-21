use serenity::client::Context;
use serenity::model::{Message, Channel};
use serenity::utils::Colour;
use serenity::client::STATE;

pub fn member_info(context: Context, message: Message, _args: Vec<String>) {
    let member = message.mentions;
    let guild_id = match STATE.lock().unwrap().get_channel(message.channel_id) {
        Some(Channel::Public(channel)) => channel.guild_id,
        Some(_) => {
            let _ = context.say("Can't use this command here");
            return;
        }
        None => {
            let _ = context.say("Can't find guild");
            return;
        }
    };
    let b = match context.get_member(guild_id, member[0].id) {
        Ok(member) => member,
        Err(_) => {
            let _ = context.say("Failed to get member");
            return;
        }
    };
    let avatar = match b.user.avatar_url() {
        Some(avatar) => avatar,
        None => {
            return;
        }
    };
    let roles = match b.roles() {
        Some(role) => role,
        None => {
            return;
        }
    };
    let ayy: Vec<String> = roles.into_iter().map(|x| x.name).collect();
    let _ = context.send_message(message.channel_id, |m| {
        m.embed(|e| {
            e.colour(Colour::blue())
                .title(&format!("Info on {}", b.user.name))
                .field(|f| {
                    f.inline(true)
                        .name("Name")
                        .value(&b.user.name)
                })
                .field(|f| {
                    f.inline(true)
                        .name("ID")
                        .value(&format!("{}", b.user.id))
                })
                .field(|f| {
                    f.inline(false)
                        .name("Roles")
                        .value(&format!("{}", ayy.join(", ")))
                })
                .thumbnail(|f| f.url(&avatar))
        })
    });

}
