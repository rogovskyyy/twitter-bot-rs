use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub uri: String,
    pub key: String,
    pub ids: String,
    pub interval: String,
    pub convert: String,
    pub per_page: String,
    pub page: String,
    pub sort: String,
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub secret_access_token: String,
}
