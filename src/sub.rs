use anyhow::Result;
use dotenvy::dotenv;
use std::env;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let queue_name = env::var("QUEUE_NAME")?;

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe(&queue_name).await?;

    println!("Subscribed to channel: {}", queue_name);

    let mut message_stream = pubsub.on_message();

    while let Some(msg) = message_stream.next().await {
        let payload: String = msg.get_payload()?;
        println!("Received: {}", payload);
    }

    Ok(())
}
