pub mod auth;
pub mod relation;
pub mod search;
pub mod types;
pub mod types_resp;
use reqwest::Client;

pub const LOGIN_URL: &str = "https://api.twitter.com/1.1/onboarding/task.json";
pub const LOGOUR_URL: &str = "https://api.twitter.com/1.1/account/logout.json";
pub const GUEST_ACTIVE_URL: &str = "https://api.twitter.com/1.1/guest/activate.json";
pub const VERIFY_CREDENTIALS_URL: &str =
    "https://api.twitter.com/1.1/account/verify_credentials.json";
pub const OAUTH_URL: &str = "https://api.twitter.com/oauth2/token";
pub const BEARER_TOKEN: &str = "AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA";
pub const APP_CONSUMER_KEY: &str = "3nVuSoBZnx6U4vzUxf5w";
pub const APP_CONSUMER_SECRET: &str = "Bcs59EFbbsdF6Sl9Ng71smgStWEGwXXKSjYvPVt7qys";

#[derive(Debug, Clone)]
pub struct ReAPI {
    client: Client,
    guest_token: String,
    csrf_token: String,
}

#[cfg(test)]
mod tests {
    use super::{
        types::{Data, Tweet},
        *,
    };

    async fn login(api: &mut ReAPI) -> Result<String, String> {
        dotenv::dotenv().ok();
        let name = std::env::var("TWITTER_USER_NAME").unwrap();
        let pwd = std::env::var("TWITTER_USER_PASSWORD").unwrap();
        api.login(&name, &pwd, "").await
    }

    async fn search(api: &mut ReAPI) -> Result<Data, reqwest::Error> {
        let content = "@shareverse_bot";
        let limit = 50;
        let cursor = "";
        api.search(content, limit, cursor).await
    }

    async fn search_tweets(api: &mut ReAPI) -> Result<(Vec<Tweet>, String), reqwest::Error> {
        let content = "@shareverse_bot";
        let limit = 50;
        let cursor = "";
        api.search_tweets(content, limit, cursor).await
    }

    #[tokio::test]
    async fn test() {
        let mut api = ReAPI::new();
        let loggined = login(&mut api).await;
        assert!(loggined.is_ok());

        let is_logged_in = api.is_logged_in().await;
        assert!(is_logged_in);

        // let result = search(&mut api).await;
        // assert!(result.is_ok());

        // let res = search_tweets(&mut api).await;
        // assert!(res.is_ok());

        // let (tweets, _) = res.unwrap();
        // assert!(tweets.len() == 0);
    }
}
