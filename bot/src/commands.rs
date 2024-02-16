use serenity::{client::FullEvent, builder::CreateMessage};

use crate::BotData;

type Context<'a> = poise::Context<'a, BotData, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;


#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>, 
    #[description = "What will you ask the Adachi cube?"] question: Option<String>
) -> Result<(), Error> {
    let random_file = ctx.data().folders.pick_random()?;
    let contents = format!("selected: {}", random_file.to_string_lossy());
    ctx.say(contents).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn invite(
    ctx: Context<'_>
) -> Result<(), Error> {
    let bot_id = ctx.framework().bot_id;
    let invite_url = format!("https://discord.com/api/oauth2/authorize?client_id={}&permissions=2048&scope=bot", bot_id);
    ctx.say(format!(
        "Invite Adachi Cube to your server: {}\nJoin the support server: {}", invite_url, "https://discord.gg/K2MDHBZr9j" //TODO: move this invite link somewhere else
    )).await?;
    Ok(())
}

pub async fn event_handler(
    ctx: &serenity::all::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, BotData, Error>,
    data: &BotData
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            //TODO log in analytics
            println!("ready ok!");
        },
        FullEvent::GuildCreate { guild , is_new } => {
            if is_new.unwrap_or(true) {
                let guilds = ctx.http.get_guilds(None, None).await?;
                data.analytics.update_guilds(guilds.len(), ctx.shard_id.0)?;
                alert_guild_status_changed(ctx, Some(guild.name.clone()), guild.id.get(), guilds.len(), true).await?;
            }
        },
        FullEvent::GuildDelete { incomplete , full } => {
            let guilds = ctx.http.get_guilds(None, None).await?;
            data.analytics.update_guilds(guilds.len(), ctx.shard_id.0)?;
            let guild_name_maybe = full.clone().map(|k| k.name);
            alert_guild_status_changed(ctx, guild_name_maybe, incomplete.id.get(), guilds.len(), false).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn alert_guild_status_changed(ctx: &serenity::all::Context, guild_name: Option<String>, guild_id: u64, guild_count: usize, joined: bool) -> Result<(), Error> {
    let join_status = if joined {
        "joined"
    } else {
        "left"
    };
    let message = format!("{} guild `{}`:{} [{} servers]", 
        join_status, 
        guild_name.unwrap_or(String::from("<unknown>")),
        guild_id,
        guild_count
    );
    //get owner user
    let application_info = ctx.http.get_current_application_info().await?;
    let owner = application_info.owner.unwrap();
    let channel = owner.create_dm_channel(ctx.http.clone()).await?;
    channel.send_message(
        ctx.http.clone(), 
        CreateMessage::new().content(&message)
    ).await?;
    Ok(())
}