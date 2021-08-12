use egg_mode::user::TwitterUser;
use egg_mode::Token;
use std::vec::Vec;

pub fn new_client(
    consumer_key: String,
    consumer_secret: String,
    access_token_key: String,
    access_token_secret: String,
) -> TwitterClient {
    let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
    let access_token = egg_mode::KeyPair::new(access_token_key, access_token_secret);
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };
    return TwitterClient { token: token };
}

pub struct TwitterClient {
    token: Token,
}

impl TwitterClient {
    pub async fn list_follorwers(&self, user_name: String) -> Vec<TwitterUser> {
        let mut result: Vec<TwitterUser> = Vec::new();
        let mut list = egg_mode::user::followers_of(user_name, &self.token).with_page_size(200);
        let mut resp = list.call().await.unwrap();
        result.append(&mut resp.response.users);
        while resp.next_cursor != 0 {
            list.next_cursor = resp.response.next_cursor;
            resp = list.call().await.unwrap();
            result.append(&mut resp.response.users);
        }
        return result;
    }
}
