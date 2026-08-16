use chrono::Utc;
use cron::Schedule;
use sqlx::types::ipnetwork::IpNetwork;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "scan_status")]
pub enum ScanStatus {
    #[sqlx(rename = "scheduled")]
    Scheduled,
    #[sqlx(rename = "in progress")]
    InProgress,
    #[sqlx(rename = "done")]
    Done,
}

#[derive(Clone)]
pub struct ScanEntity {
    pub id: Uuid,
    pub project_id: Uuid,
    pub target: IpNetwork,
    pub status: ScanStatus,
    pub schedule: Option<String>,
    pub created_at: OffsetDateTime,
    pub next_run_at: Option<OffsetDateTime>,
}

pub struct ScanInsert {
    pub project_id: Uuid,
    pub target: IpNetwork,
    pub schedule: Option<String>,
    pub next_run_at: Option<OffsetDateTime>,
}

impl ScanInsert {
    pub fn new(project_id: Uuid, target: IpNetwork, schedule: Option<Schedule>) -> Self {
        let next_run_at = schedule.as_ref().and_then(Self::calculate_next_run);
        Self {
            project_id,
            target,
            schedule: schedule.map(|s| s.to_string()),
            next_run_at,
        }
    }
    fn calculate_next_run(schedule: &Schedule) -> Option<OffsetDateTime> {
        schedule
            .upcoming(Utc)
            .next()
            .and_then(|next| OffsetDateTime::from_unix_timestamp(next.timestamp()).ok())
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CreateScanInput {
    pub(crate) project_id: Uuid,
    pub(crate) target: IpNetwork,
    pub(crate) schedule: Option<Schedule>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn construct_scan_insert() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_target: IpNetwork = "192.168.0.1".parse().unwrap();
        let expected_schedule_with_seconds = "0 * * * * *";
        let expected_schedule = Schedule::from_str(expected_schedule_with_seconds).unwrap();
        // Act
        let actual_scan_insert = ScanInsert::new(
            expected_project_id,
            expected_target,
            Some(expected_schedule),
        );
        // Assert
        assert!(actual_scan_insert.next_run_at.is_some());
    }

    #[test]
    fn construct_scan_insert_no_schedule() {
        // Arrange
        let expected_project_id = Uuid::now_v7();
        let expected_target: IpNetwork = "192.168.0.1".parse().unwrap();
        // Act
        let actual_scan_insert = ScanInsert::new(expected_project_id, expected_target, None);
        // Assert
        assert!(actual_scan_insert.next_run_at.is_none());
    }
}
