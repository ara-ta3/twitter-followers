use envconfig::Envconfig;
use std::process;
mod config;
mod twitter;

#[tokio::main]
async fn main() {
    let c = match config::Config::init() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let cli = twitter::new_client(
        c.twitter_consumer_key,
        c.twitter_consumer_secret,
        c.twitter_access_token_key,
        c.twitter_access_token_secret,
    );
    let users = cli.list_follorwers(String::from("ara_ta3")).await;
    println!("{}", users.len());
}
