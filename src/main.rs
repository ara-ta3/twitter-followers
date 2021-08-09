use envconfig::Envconfig;
use envconfig_derive::Envconfig;
use std::process;

#[tokio::main]
async fn main() {
    let config = match Config::init() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    let con_token =
        egg_mode::KeyPair::new(config.twitter_consumer_key, config.twitter_consumer_secret);
    let access_token = egg_mode::KeyPair::new(
        config.twitter_access_token_key,
        config.twitter_access_token_secret,
    );
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    // let u = egg_mode::user::show("rustlang", &token).await.unwrap();
    // println!("{} (@{})", u.name, u.screen_name);
    let mut list = egg_mode::user::followers_of("ara_ta3", &token).with_page_size(20);
    let resp = list.call().await.unwrap();
    for user in resp.response.users {
        println!("{} (@{})", user.name, user.screen_name);
    }
    list.next_cursor = resp.response.next_cursor;
    let resp = list.call().await.unwrap();
    for user in resp.response.users {
        println!("{} (@{})", user.name, user.screen_name);
    }
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
