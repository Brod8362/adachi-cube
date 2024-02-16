use crate::BotData;

type Context<'a> = poise::Context<'a, BotData, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;


#[poise::command(slash_command)]
pub async fn roll(
    ctx: Context<'_>, 
    #[description = "What will you ask the Adachi cube?"] question: Option<String>
) -> Result<(), Error> {
    //TODO use question as a seed
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