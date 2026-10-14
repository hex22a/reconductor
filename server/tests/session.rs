use std::sync::Arc;

use server::{
    Config,
    features::{
        csrf::model::{CsrfCookie, CsrfToken},
        session::{
            model::{SessionToken, UserSession},
            repository::{SessionRepository, SessionRepositoryError, SessionStore},
        },
        user::model::Username,
    },
    infra::persistence::kv::{self, FredKvProvider, KvConfig},
};
use uuid::Uuid;

async fn create_store() -> SessionStore<FredKvProvider> {
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
    SessionStore::new(kv)
}

fn create_user_session_fixture(token: &SessionToken) -> UserSession {
    UserSession {
        token: token.to_owned(),
        user_id: Uuid::now_v7(),
        username: Username::new("testuser"),
        csrf_token: CsrfToken::new("csrf_token"),
        csrf_cookie: CsrfCookie::new("csrf_cookei"),
    }
}

#[tokio::test]
async fn stores_session_under_correct_key() {
    // Arrange
    let store = create_store().await;
    let expected_token = SessionToken::new("store_under_correct_key");
    let expected_user_session = create_user_session_fixture(&expected_token);

    // Act
    let actual_result = store.create_user_session(expected_user_session).await;

    //Assert
    assert!(actual_result.is_ok());
}

#[tokio::test]
async fn returns_not_found_error_for_missing_session() {
    // Arrange
    let store = create_store().await;
    let expected_token = SessionToken::new("does_not_exist");

    // Act
    let actual_result = store.get_user_session(&expected_token).await;

    // Assert
    assert!(matches!(
        actual_result,
        Err(SessionRepositoryError::NotFound)
    ));
}

#[tokio::test]
async fn gets_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = SessionToken::new("get_session_from_storage");
    let expected_user_session = create_user_session_fixture(&expected_token);
    store
        .create_user_session(expected_user_session.clone())
        .await
        .unwrap();

    // Act
    let actual_user_session = store.get_user_session(&expected_token).await.unwrap();

    // Assert
    assert_eq!(actual_user_session, expected_user_session);
}

#[tokio::test]
async fn deletes_session_from_storage() {
    // Arrange
    let store = create_store().await;
    let expected_token = SessionToken::new("token_to_delete");
    let expected_user_session = create_user_session_fixture(&expected_token);
    store
        .create_user_session(expected_user_session)
        .await
        .unwrap();

    // Act
    store.delete_user_session(&expected_token).await.unwrap();

    // Assert
    let result = store.get_user_session(&expected_token).await;
    assert!(matches!(result, Err(SessionRepositoryError::NotFound)));
}
