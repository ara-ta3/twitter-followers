use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use std::process;

fn main() {
    let config = match Config::init() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    println!("{:#?}", config)
}

#[derive(Envconfig, Debug)]
struct Config {
    #[envconfig(from = "TWITTER_CONSUMER_KEY")]
    twitter_consumer_key: String,
    #[envconfig(from = "TWITTER_CONSUMER_SECRET")]
    twitter_consumer_secret: String,
    #[envconfig(from = "TWITTER_ACCESS_TOKEN_KEY")]
    twitter_access_token_key: String,
    #[envconfig(from = "TWITTER_ACCESS_TOKEN_SECRET")]
    twitter_access_token_secret: String,
}
