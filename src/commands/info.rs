extern crate chrono;

use self::chrono::*;
use serenity::client::Context;
use serenity::model::{Message, Channel};
use serenity::utils::Colour;
use serenity::client::STATE;

pub fn member_info(context: Context, message: Message, _args: Vec<String>) {
    let member = message.mentions;
    if member.is_empty() {
        let _ = context.say("You must mention someome to view info about them.");
    }

    let guild_id = match context.get_channel(message.channel_id) {
        Ok(Channel::Public(channel)) => channel.guild_id,
        Ok(_) => {
            let _ = context.say("Can't use this command here");
            return;
        }
        Err(_) => {
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

    let timestamp = NaiveDateTime::from_timestamp(b.user.created_at().sec, 0)
        .format("%m/%d/%Y")
        .to_string();

    let ayy: Vec<String> = roles.into_iter().map(|x| x.name).collect();
    let _ = context.send_message(message.channel_id, |m| {
        m.embed(|e| {
            e.colour(Colour::blue())
                .title(&format!("Info on {}", b.user.name))
                .field(|f| {
                    f.inline(true)
                        .name("Name")
                        .value(&format!("{}#{}", b.user.name, b.user.discriminator))
                })
                .field(|f| {
                    f.inline(true)
                        .name("ID")
                        .value(&format!("{}", b.user.id))
                })
                .field(|f| {
                    f.inline(false)
                        .name("Roles")
                        .value(&ayy.join(", "))
                })
                .field(|f| {
                    f.inline(true)
                        .name("Creation Date")
                        .value(&format!("{}", timestamp))
                })
                .field(|f| {
                    f.inline(true)
                        .name("Join Date")
                        .value(&format!("{}/{}/{}",
                                        b.joined_at.parse::<DateTime<UTC>>().unwrap().month(),
                                        b.joined_at.parse::<DateTime<UTC>>().unwrap().day(),
                                        b.joined_at.parse::<DateTime<UTC>>().unwrap().year()))
                })
                .thumbnail(|f| f.url(&avatar))
        })
    });
}

pub fn guild_info(context: Context, message: Message, _args: Vec<String>) {
    let id = match context.get_channel(message.channel_id) {
        Ok(Channel::Public(channel)) => channel.guild_id,
        Ok(_) => {
            let _ = context.say("Can't use this commands in groups nor dm's.");
            return;
        }
        Err(_) => {
            let _ = context.say("Can't find guild");
            return;
        }
    };
    let state = STATE.lock().unwrap();
    let guild = match state.get_guild(id).clone() {
        Some(guild) => guild,
        None => {
            let _ = context.say("Failed to get guild");
            return;
        }
    };
    drop(&state);
    let guild_icon = match guild.icon_url() {
        Some(avatar) => avatar,
        None => {
            return;
        }
    };
    let owner = match guild.get_member(guild.owner_id) {
        Some(owner) => owner,
        None => return,
    };
    let roles: Vec<_> = guild.roles.values().map(|x| &x.name).collect();
    let _ = context.send_message(message.channel_id, |m| {
        m.embed(|e| {
            e.colour(Colour::from_rgb(233, 30, 99))
                .title(&format!("Guild info for {}", guild.name))
                .field(|f| {
                    f.inline(true)
                        .name("Owner")
                        .value(&owner.user.name)
                })
                .field(|f| {
                    f.inline(true)
                        .name("ID")
                        .value(&format!("{}", guild.id))
                })
                .field(|f| {
                    f.inline(true)
                        .name("Region")
                        .value(&guild.region)
                })
                .field(|f| {
                    f.inline(false)
                        .name("Total Roles")
                        .value(&format!("{}", roles.len()))
                })
                .thumbnail(|f| f.url(&guild_icon))
        })
    });
}
