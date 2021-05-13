mod config;
mod cryptobot;

use crate::cryptobot::CryptoBot;
use rust_twitter_bot_lib::{Tweet, TwitterBot};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::*;

#[tokio::main]
async fn main() {
    let data = CryptoBot::send().await;
    let json = CryptoBot::load_json("config.json").await;

    match json {
        Ok(v) => {
            let example_bot = TwitterBot::new()
                .consumer_key(&v.consumer_key)
                .consumer_secret_key(&v.consumer_secret)
                .access_token(&v.access_token)
                .secret_access_token(&v.secret_access_token);
    
            match example_bot.tweet(&data, None) {
                Ok(_) => println!("Success"),
                Err(e) => println!("{}", e)
            };
        },
        Err(f) => panic!("{}", f)
    }
}
