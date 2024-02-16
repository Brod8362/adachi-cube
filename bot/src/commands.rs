use poise::CreateReply;
use serenity::{client::FullEvent, builder::{CreateMessage, CreateAttachment}};

use crate::BotData;

type Context<'a> = poise::Context<'a, BotData, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;


#[poise::command(slash_command)]
pub async fn ask(
    ctx: Context<'_>, 
    #[description = "What will you ask the Adachi cube?"] question: Option<String>
) -> Result<(), Error> {
    let random_file = ctx.data().folders.pick_random()?;
    let mut attachment = CreateAttachment::path(random_file.as_path()).await?;
    let extension: String = if let Some(ext) = random_file.extension() {
        ext.to_string_lossy().into()
    } else {
        String::from("mp4")
    };
    attachment.filename = format!("judgement.{}", extension);
    let content: String = if let Some(q) = question {
        let mut q = q.replace("`", "").replace("@", "");
        if !q.ends_with('?') {
            q.push('?');
        }
        format!("The Adachi cube has spoken: `{}`", q)
    } else {
        String::from("The Adachi cube has spoken.")
    };
    let reply = CreateReply {..Default::default()}
        .attachment(attachment)
        .content(content);
    ctx.send(reply).await?;     
    ctx.data().analytics.update_usage(ctx.guild_id().unwrap().get(), ctx.channel_id().get()).await?;   
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
        FullEvent::Ready { data_about_bot: _, .. } => {
            //TODO log in analytics
            data.analytics.info("online").await?;
        },
        FullEvent::GuildCreate { guild , is_new } => {
            if is_new.unwrap_or(true) {
                let guilds = ctx.http.get_guilds(None, None).await?;
                data.analytics.update_guilds(guilds.len(), ctx.shard_id.0).await?;
                alert_guild_status_changed(ctx, Some(guild.name.clone()), guild.id.get(), guilds.len(), true).await?;
            }
        },
        FullEvent::GuildDelete { incomplete , full } => {
            let guilds = ctx.http.get_guilds(None, None).await?;
            data.analytics.update_guilds(guilds.len(), ctx.shard_id.0).await?;
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