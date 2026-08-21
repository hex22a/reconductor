use std::sync::Arc;

use server::features::port::repository::{PgPortRepository, PortRepository};
use sqlx::PgPool;
use uuid::Uuid;

async fn setup_port(db: &PgPool) -> (Uuid, Uuid, Uuid, Uuid, Uuid) {
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
    let expected_scan_run_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.scan_runs
            (scan_id)
        VALUES ($1)
        RETURNING id
        "#,
    )
    .bind(expected_scan_id)
    .fetch_one(db)
    .await
    .unwrap();
    let expected_host_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.scan_hosts
            (scan_run_id, ip)
        VALUES ($1, '192.168.0.1')
        RETURNING id
        "#,
    )
    .bind(expected_scan_run_id)
    .fetch_one(db)
    .await
    .unwrap();
    let expected_port_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO recon.scan_ports
            (host_id, port)
        VALUES ($1, 22)
        RETURNING id
        "#,
    )
    .bind(expected_host_id)
    .fetch_one(db)
    .await
    .unwrap();
    (
        expected_port_id,
        expected_host_id,
        expected_scan_run_id,
        expected_scan_id,
        expected_project_id,
    )
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_scan_by_id(db: PgPool) {
    // Arrange
    let (
        expected_port_id,
        _expected_host_id,
        _expected_scan_run_id,
        _expected_scan_id,
        _expected_project_id,
    ) = setup_port(&db).await;
    let repo = PgPortRepository::new(Arc::new(db));
    // Act
    let actual_port = repo.get_port(&expected_port_id).await.unwrap();
    // Assert
    assert_eq!(actual_port.id, expected_port_id);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_scan_by_id_not_found(db: PgPool) {
    // Arrange
    let expected_port_id = Uuid::now_v7();
    let repo = PgPortRepository::new(Arc::new(db));
    // Act
    let actual_result = repo.get_port(&expected_port_id).await;
    // Assert
    assert!(matches!(actual_result, Err(sqlx::Error::RowNotFound)));
}

#[sqlx::test(migrations = "../migrations")]
async fn test_list_scans(db: PgPool) {
    // Arrange
    let expected_limit: i64 = 1;
    let (
        _expected_port_id,
        expected_host_id,
        _expected_scan_run_id,
        _expected_scan_id,
        _expected_project_id,
    ) = setup_port(&db).await;
    let repo = PgPortRepository::new(Arc::new(db));
    // Act
    let actual_projects = repo
        .list_ports(&expected_host_id, None, expected_limit)
        .await
        .unwrap();
    // Assert
    assert_eq!(actual_projects.len(), expected_limit as usize);
}
