use std::sync::Arc;

use server::{
    Config,
    features::csrf::repository::{CsrfRepository, CsrfStore},
    infra::persistence::kv::{self, FredKvProvider, KvConfig},
};

async fn create_store() -> CsrfStore<FredKvProvider> {
    dotenvy::dotenv().ok();
    let config = Config::from_env().expect("Unable to read environment variables");
    let kv = Arc::new(
        kv::init_kv(KvConfig {
            username: config.kv_username,
            password: config.kv_password,
            host: config.kv_host,
            port: config.kv_port,
            database: config.kv_db,
        })
        .await,
    );
    CsrfStore::new(kv)
}

#[tokio::test]
async fn stores_csrf_under_correct_key() {
    // Arrange
    let store = create_store().await;
    let expected_token = "store_csrf_under_correct_key";

    // Act
    let actual_result = store.create_anonymous_csrf(expected_token).await.unwrap();

    //Assert
    assert_eq!(actual_result, ());
}

#[tokio::test]
async fn returns_false_for_missing_csrf() {
    // Arrange
    let store = create_store().await;
    let expected_token = "csrf_does_not_exist";

    // Act
    let actual_valid = store.verify_anonymous_csrf(expected_token).await.unwrap();

    // Assert
    assert!(!actual_valid);
}

#[tokio::test]
async fn gets_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = "get_csrf_from_storage";
    store.create_anonymous_csrf(expected_token).await.unwrap();

    // Act
    let actual_valid = store.verify_anonymous_csrf(expected_token).await.unwrap();

    // Assert
    assert!(actual_valid);
}

#[tokio::test]
async fn deletes_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = "csrf_token_to_delete";
    store.create_anonymous_csrf(expected_token).await.unwrap();

    // Act
    store.delete_anonymous_csrf(expected_token).await.unwrap();

    // Assert
    let actual_valid = store.verify_anonymous_csrf(expected_token).await.unwrap();
    assert!(!actual_valid);
}
