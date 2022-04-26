use serde::Deserialize;
use yodlee_rs::{user::UserRegistration, Client};

#[derive(Deserialize, Clone, Debug)]
struct Config {
    api_endpoint: String,
    api_version: String,
    admin_login_name: String,
    client_id: String,
    client_secret: String,
    test_user1: String,
}

fn make_client() -> (Config, Client) {
    let config = envy::prefixed("YODLEE_").from_env::<Config>().unwrap();

    (
        config.clone(),
        Client::new(
            config.api_endpoint,
            config.api_version,
            config.admin_login_name,
            config.client_id,
            config.client_secret,
        ),
    )
}

#[tokio::test]
async fn test_client_open() {
    let (_, mut client) = make_client();
    let _ = client.open().await.unwrap();
    assert!(client.is_open());
    client.close().await.unwrap();
}

#[tokio::test]
async fn test_get_user_details() {
    let (config, mut client) = make_client();
    let _ = client.open().await.unwrap();

    let mut user = client.user(config.test_user1.clone()).await.unwrap();
    let res = user.get_details().await.unwrap();
    assert_eq!(res.user.login_name, config.test_user1);
    assert!(res.user.id.is_some());

    client.close().await.unwrap();
}

// Disabling test because user registration in sandbox is not supported.
//#[tokio::test]
async fn _test_user_registration() {
    let (_, mut client) = make_client();
    let _ = client.open().await.unwrap();

    let user_registration = UserRegistration {
        login_name: "test_user5".to_string(),
        ..Default::default()
    };

    let res = client.register_user(user_registration).await.unwrap();

    println!("{:#?}", res);

    client.close().await.unwrap();
}
