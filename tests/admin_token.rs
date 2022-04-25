use serde::Deserialize;
use yodlee_rs::Client;

#[derive(Deserialize, Debug)]
struct Config {
    api_endpoint: String,
    api_version: String,
    admin_login_name: String,
    client_id: String,
    client_secret: String,
}

#[tokio::test]
async fn test_admin_token() {
    let config = envy::prefixed("YODLEE_").from_env::<Config>().unwrap();

    let client = Client::new(
        config.api_endpoint,
        config.api_version,
        config.admin_login_name,
        config.client_id,
        config.client_secret,
    );

    let _ = client.open().await.unwrap();
    let admin_token = client.admin_token();
    assert!(admin_token.is_some());
    assert!(!admin_token.unwrap().is_empty());

    client.close().await.unwrap();
}
