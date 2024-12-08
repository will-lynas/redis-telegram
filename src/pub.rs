use anyhow::Result;
use dotenvy::dotenv;
use redis::AsyncCommands;
use std::{env, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let channel = env::var("CHANNEL")?;

    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_multiplexed_tokio_connection().await?;

    let mut counter = 1;
    loop {
        let message = counter.to_string();
        con.publish::<_, _, ()>(&channel, &message).await?;
        println!("Published: {:#?}", message);

        counter += 1;
        sleep(Duration::from_secs(2)).await;
    }
}
