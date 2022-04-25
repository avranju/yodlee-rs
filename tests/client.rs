use serde::Deserialize;
use yodlee_rs::{user::UserRegistration, Client};

#[derive(Deserialize, Debug)]
struct Config {
    api_endpoint: String,
    api_version: String,
    admin_login_name: String,
    client_id: String,
    client_secret: String,
}

fn make_client() -> Client {
    let config = envy::prefixed("YODLEE_").from_env::<Config>().unwrap();

    Client::new(
        config.api_endpoint,
        config.api_version,
        config.admin_login_name,
        config.client_id,
        config.client_secret,
    )
}

#[tokio::test]
async fn test_admin_token() {
    let client = make_client();
    let _ = client.open().await.unwrap();
    let admin_token = client.admin_token();
    assert!(admin_token.is_some());
    assert!(!admin_token.unwrap().is_empty());
    client.close().await.unwrap();
}

// Disabling test because user registration in sandbox is not supported.
//#[tokio::test]
async fn _test_user_registration() {
    let client = make_client();
    let _ = client.open().await.unwrap();

    let user_registration = UserRegistration {
        login_name: "test_user1".to_string(),
        ..Default::default()
    };

    let user = client.user().unwrap();
    let res = user.register(user_registration).await.unwrap();

    println!("{:#?}", res);

    client.close().await.unwrap();
}
