use std::sync::Arc;

use server::features::scan::{
    model::ScanInsert,
    repository::{PgScanRespository, ScanRepository},
};
use sqlx::PgPool;
use time::macros::datetime;
use uuid::Uuid;

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

async fn setup_scan(db: &PgPool) -> (Uuid, Uuid) {
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
    let expected_scan_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.scans
            (project_id, target, schedule)
        VALUES ($1, '192.168.0.1', '* * * * *')
        RETURNING id
        "#,
    )
    .bind(expected_project_id)
    .fetch_one(db)
    .await
    .unwrap();
    return (expected_scan_id, expected_project_id);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_create_scan(db: PgPool) {
    // Arrange
    let (expected_project_id, _expected_owner_id) = setup_project(&db).await;
    let expected_next_run_at = datetime!(2019-01-01 0:00 UTC);
    let expected_target = "192.168.0.1".try_into().unwrap();
    let expected_schedule = "* * * * *".to_string();
    let expected_scan_insert = ScanInsert {
        project_id: expected_project_id,
        target: expected_target,
        schedule: Some(expected_schedule),
        next_run_at: Some(expected_next_run_at),
    };
    let repo = PgScanRespository::new(Arc::new(db));
    // Act
    let actual_scan_entity = repo.create_scan(expected_scan_insert).await.unwrap();
    // Assert
    assert_eq!(actual_scan_entity.project_id, expected_project_id,);
    assert_eq!(actual_scan_entity.target, expected_target);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_scan_by_id(db: PgPool) {
    // Arrange
    let (expected_scan_id, _expected_project_id) = setup_scan(&db).await;
    let repo = PgScanRespository::new(Arc::new(db));
    // Act
    let actual_scan = repo.get_scan(&expected_scan_id).await.unwrap();
    // Assert
    assert_eq!(actual_scan.id, expected_scan_id);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_scan_by_id_not_found(db: PgPool) {
    // Arrange
    let expected_scan_id = Uuid::now_v7();
    let repo = PgScanRespository::new(Arc::new(db));
    // Act
    let actual_result = repo.get_scan(&expected_scan_id).await;
    // Assert
    assert!(matches!(actual_result, Err(sqlx::Error::RowNotFound)));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_list_scans(db: PgPool) {
    // Arrange
    let expected_limit: i64 = 1;
    let (expected_project_id, _expected_owner_id) = setup_project(&db).await;
    let expected_next_run_at = datetime!(2019-01-01 0:00 UTC);
    let expected_target = "192.168.0.1".try_into().unwrap();
    let expected_schedule = "* * * * *".to_string();
    let expected_scan_insert = ScanInsert {
        project_id: expected_project_id,
        target: expected_target,
        schedule: Some(expected_schedule),
        next_run_at: Some(expected_next_run_at),
    };
    let repo = PgScanRespository::new(Arc::new(db));
    repo.create_scan(expected_scan_insert).await.unwrap();
    // Act
    let actual_projects = repo
        .list_scans(&expected_project_id, None, expected_limit)
        .await
        .unwrap();
    // Assert
    assert_eq!(actual_projects.len(), expected_limit as usize);
}
