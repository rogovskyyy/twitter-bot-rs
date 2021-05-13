extern crate chrono;
use crate::config::Config;
use chrono::prelude::*;
use serde_json::{Result, Value};
use std::{fs::File, io::BufReader, path::Path};

pub struct CryptoBot;

impl CryptoBot {
    pub async fn send() -> String {
        let link_parser = CryptoBot::builder().await;
        match reqwest::get(link_parser).await {
            Ok(v) => match v.text().await {
                Ok(w) => CryptoBot::json_parser(w),
                Err(_) => panic!("Could not parse response properly"),
            },
            Err(_) => panic!("System could not get reponse from API"),
        }
        .await
    }

    pub async fn builder() -> String {
        let cfg = CryptoBot::load_json("config.json").await.unwrap();
        let string_builder = format!(
            "{}?&key={}&ids={}&interval={}&convert={}&per-page={}&page={}&sort={}",
            cfg.uri, cfg.key, cfg.ids, cfg.interval, cfg.convert, cfg.per_page, cfg.page, cfg.sort
        );
        string_builder
    }

    pub async fn json_parser(data: String) -> String {
        let v: Vec<Value> = serde_json::from_str(&data).unwrap();
        let mut builder = String::new();

        let time = Utc::now();

        let formater = format!(
            "Top 10 crypto for {} (UTC) \n\n",
            time.format("%b %d, %Y %H:%M").to_string()
        );

        builder.push_str(&formater);
        let mut i = 0;

        for item in &v {
            if i >= 10 {
                break;
            }

            let val: String = item["price"].as_str().unwrap().to_string();
            let convert: f64 = val.parse().unwrap();

            builder.push_str(&format!(
                "{} @ #{} : ${:.2} \n",
                item["rank"].as_str().unwrap(),
                item["id"].as_str().unwrap(),
                convert
            ));
            i = i + 1;
        }

        builder.push_str(&"\n #btc #crypto \n");

        builder
    }

    pub async fn load_json<P: AsRef<Path>>(path: P) -> Result<Config> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let u = serde_json::from_reader(reader)?;
        Ok(u)
    }
}
