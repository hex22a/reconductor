use std::sync::Arc;

use server::features::project::{
    model::{ProjectEntity, ProjectInsert},
    repository::{PgProjectRepository, ProjectRepository},
};
use sqlx::PgPool;
use time::macros::datetime;
use uuid::Uuid;

async fn setup_user(db: &PgPool) -> Uuid {
    let expected_user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.users
            (username, password_hash)
        VALUES ('test_user', 'password_hash')
        RETURNING id
        "#,
    )
    .fetch_one(db)
    .await
    .unwrap();
    return expected_user_id;
}

async fn setup_project(db: &PgPool) -> (Uuid, Uuid) {
    let expected_user_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.users
            (username, password_hash)
        VALUES ('test_user', 'password_hash')
        RETURNING id
        "#,
    )
    .fetch_one(db)
    .await
    .unwrap();
    let expected_project_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.projects
            (name, owner_id)
        VALUES ('test_project', $1)
        RETURNING id
        "#,
    )
    .bind(expected_user_id)
    .fetch_one(db)
    .await
    .unwrap();
    return (expected_project_id, expected_user_id);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_project(db: PgPool) {
    // Arrange
    let expected_project_id = Uuid::now_v7();
    let expected_owner_id = setup_user(&db).await;
    let expected_name = "test".to_string();
    let expected_created_at = datetime!(2019-01-01 0:00);
    let expected_project_entity = ProjectEntity {
        id: expected_project_id,
        owner_id: expected_owner_id,
        name: expected_name.clone(),
        created_at: expected_created_at,
    };
    let expected_project_insert = ProjectInsert {
        owner_id: expected_owner_id,
        name: expected_name,
    };
    let repo = PgProjectRepository::new(Arc::new(db));
    // Act
    let actual_project_entity = repo.create_project(expected_project_insert).await.unwrap();
    // Assert
    assert_eq!(
        actual_project_entity.owner_id,
        expected_project_entity.owner_id
    );
    assert_eq!(actual_project_entity.name, expected_project_entity.name);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_project_by_id(db: PgPool) {
    // Arrange
    let (expected_project_id, expected_owner_id) = setup_project(&db).await;
    let repo = PgProjectRepository::new(Arc::new(db));
    // Act
    let actual_project = repo
        .get_project(&expected_project_id, &expected_owner_id)
        .await
        .unwrap();
    // Assert
    assert_eq!(actual_project.id, expected_project_id);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_project_by_id_wrong_owner(db: PgPool) {
    // Arrange
    let (expected_project_id, _) = setup_project(&db).await;
    let expected_owner_id = Uuid::now_v7();
    let repo = PgProjectRepository::new(Arc::new(db));
    // Act
    let actual_result = repo
        .get_project(&expected_project_id, &expected_owner_id)
        .await;
    // Assert
    assert!(matches!(actual_result, Err(sqlx::Error::RowNotFound)));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_project_by_id_not_found(db: PgPool) {
    // Arrange
    let expected_project_id = Uuid::now_v7();
    let expected_owner_id = Uuid::now_v7();
    let repo = PgProjectRepository::new(Arc::new(db));
    // Act
    let actual_result = repo
        .get_project(&expected_project_id, &expected_owner_id)
        .await;
    // Assert
    assert!(matches!(actual_result, Err(sqlx::Error::RowNotFound)));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_list_projects(db: PgPool) {
    // Arrange
    let expected_limit: i64 = 1;
    let expected_owner_id = setup_user(&db).await;
    let expected_name = "test".to_string();
    let expected_project_insert = ProjectInsert {
        owner_id: expected_owner_id,
        name: expected_name,
    };
    let repo = PgProjectRepository::new(Arc::new(db));
    repo.create_project(expected_project_insert).await.unwrap();
    // Act
    let actual_projects = repo
        .list_projects(&expected_owner_id, None, expected_limit)
        .await
        .unwrap();
    // Assert
    assert_eq!(actual_projects.len(), expected_limit as usize);
}
