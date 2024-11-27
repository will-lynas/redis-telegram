use anyhow::Result;
use dotenvy::dotenv;
use redis::AsyncCommands;
use schema::Message;
use std::{env, time::Duration};
use tokio::time::sleep;

mod schema;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let channel = env::var("CHANNEL")?;
    let chat_id = env::var("CHAT_ID")?;

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_multiplexed_tokio_connection().await?;

    let mut counter = 1;
    loop {
        let message = &Message {
            text: counter.to_string(),
            is_markdown: false,
            chat_id: chat_id.parse()?,
            thread_id: None,
        };
        let message_ser = serde_json::to_string(&message)?;
        con.publish::<_, _, ()>(&channel, &message_ser).await?;
        println!("Published: {:#?}", message);

        counter += 1;
        sleep(Duration::from_secs(2)).await;
    }
}
