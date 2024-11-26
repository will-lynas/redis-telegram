use anyhow::Result;
use dotenvy::dotenv;
use std::env;

fn main() -> Result<()> {
    dotenv()?;

    let bot_token = env::var("BOT_TOKEN")?;
    let queue_name = env::var("QUEUE_NAME")?;
    let chat_id = env::var("CHAT_ID")?;

    println!("{bot_token}, {queue_name}, {chat_id}");

    Ok(())
}
