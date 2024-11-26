use anyhow::Result;
use dotenvy::dotenv;
use std::{env, time::Duration};
use teloxide::{prelude::Requester, Bot, RequestError};
use tokio::time::sleep;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let channel = env::var("CHANNEL")?;
    let chat_id = env::var("CHAT_ID")?;

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe(&channel).await?;

    println!("Subscribed to channel: {}", channel);

    let mut message_stream = pubsub.on_message();

    let bot = Bot::from_env();

    while let Some(msg) = message_stream.next().await {
        let payload: String = msg.get_payload()?;
        println!("Received: {}", &payload);
        while let Err(RequestError::RetryAfter(seconds)) =
            bot.send_message(chat_id.clone(), &payload).await
        {
            println!("Rate limited. Retrying in {seconds} seconds");
            sleep(Duration::from_secs(1)).await;
        }
    }

    Ok(())
}
