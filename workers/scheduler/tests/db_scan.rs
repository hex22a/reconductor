use scheduler::db::scan::{PgScanRepository, ScanRepository};
use sqlx::PgPool;

#[sqlx::test(migrations = "../../migrations/")]
async fn test_fetch_due_scans_return_due_scans(db: PgPool) {
    // Arrange
    let repo = PgScanRepository { db: db.clone() };
    let user_id = sqlx::query_scalar!(
        r#"
        INSERT INTO recon.users
            (username, password_hash)
        VALUES ('test_user', 'password_hash')
        RETURNING id
        "#
    )
    .fetch_one(&db)
    .await
    .unwrap();
    let project_id = sqlx::query_scalar!(
        r#"
        INSERT INTO recon.projects
            (name, owner_id)
        VALUES ('test_project', $1)
        RETURNING id
        "#,
        user_id,
    )
    .fetch_one(&db)
    .await
    .unwrap();
    sqlx::query!(
        r#"
        INSERT INTO recon.scans
            (project_id, target, next_run_at)
        VALUES ($1, '192.168.50.0/16', NOW() - INTERVAL '1 minute')
        "#,
        project_id,
    ).execute(&db).await.unwrap();
    println!("uid: {}", project_id);
    // Act
    let scans = repo.fetch_due_scans().await.unwrap();
    // Assert
    assert_eq!(scans.len(), 1);
}
