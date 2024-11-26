use anyhow::Result;
use dotenvy::dotenv;
use redis::AsyncCommands;
use std::{env, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let queue_name = env::var("QUEUE_NAME")?;

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_multiplexed_tokio_connection().await?;

    let mut counter = 1;
    loop {
        let message = format!("Message {}", counter);
        con.publish::<_, _, ()>(&queue_name, &message).await?;
        println!("Published: {}", message);

        counter += 1;
        sleep(Duration::from_secs(1)).await;
    }
}
