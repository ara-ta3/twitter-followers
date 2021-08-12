use envconfig::Envconfig;
use envconfig_derive::Envconfig;

#[derive(Envconfig, Debug)]
pub struct Config {
    #[envconfig(from = "TWITTER_CONSUMER_KEY")]
    pub twitter_consumer_key: String,
    #[envconfig(from = "TWITTER_CONSUMER_SECRET")]
    pub twitter_consumer_secret: String,
    #[envconfig(from = "TWITTER_ACCESS_TOKEN_KEY")]
    pub twitter_access_token_key: String,
    #[envconfig(from = "TWITTER_ACCESS_TOKEN_SECRET")]
    pub twitter_access_token_secret: String,
}
