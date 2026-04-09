use std::env;
use dotenv::dotenv;
use serenity::Client;
use serenity::prelude::GatewayIntents;
use crate::error::BotError;

mod error;

#[tokio::main]
async fn main() -> Result<(), BotError> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MEMBERS;
    let mut client = Client::builder(&token, intents).await.map_err(Box::new)?;
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
    Ok(())
}
