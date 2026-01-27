use crate::server_utils::configuration::Settings;
use crate::server_utils::logging::Logger;
use chrono::{NaiveDateTime, TimeDelta};
use log::{log, Level};
use sqlx::{query, Pool, Postgres};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

/// The background task handling session cleanup in the database.
async fn session_cleanup_task(expiry_mins: u8, db_pool: Pool<Postgres>) {
    let now = chrono::Utc::now();
    // delete sessions with twice the expiry time ago
    let ready_to_delete = now - TimeDelta::minutes((expiry_mins * 2) as i64);
    let ready_to_delete: NaiveDateTime =
        NaiveDateTime::new(ready_to_delete.date_naive(), ready_to_delete.time());
    let query_result = query!(
        r#"
                    DELETE FROM session
                    WHERE expires_at < $1 ;
                    "#,
        ready_to_delete
    )
    .execute(&db_pool)
    .await;

    match query_result {
        Err(e) => {
            log!(Level::Warn, "Failed to cleanup sessions: {}", e);
        }
        Ok(_) => {
            log!(Level::Debug, "cleaned up sessions");
        }
    }
}

/// Sets up the scheduler for background tasks.
pub async fn setup_scheduler(
    db_pool: Pool<Postgres>,
    config: Settings,
) -> Result<JobScheduler, JobSchedulerError> {
    let expiry_mins = config.authorization.session_expiry_mins;
    let session_cleanup_cron_string = format!("0 0/{} * * * *", expiry_mins);
    let db_pool = db_pool.clone();
    let scheduler = JobScheduler::new().await?;
    scheduler.start().await?;

    // session cleanup
    let session_cleanup_job = Job::new_async(session_cleanup_cron_string, move |_uuid, _l| {
        let db_pool = db_pool.clone();
        Box::pin(async move {
            session_cleanup_task(expiry_mins, db_pool).await;
        })
    })?;
    scheduler.add(session_cleanup_job).await?;

    // logfile cleanup
    let logfile_cleanup_cron_string = "0 5 0 * * * *";
    let log_settings = config.log.clone();
    let logfile_cleanup_job = Job::new_async(logfile_cleanup_cron_string, move |_uuid, _l| {
        let log_settings = log_settings.clone();
        Box::pin(async move {
            Logger::delete_outdated_log_files(&log_settings, false).await;
        })
    })?;
    scheduler.add(logfile_cleanup_job).await?;

    // use a new logfile
    let new_logfile_cron_string = "0 1 0 * * * *";
    let new_logfile_job = Job::new_async(new_logfile_cron_string, move |_uuid, _l| {
        Box::pin(async move {
            Logger::set_new_logfile().await;
        })
    })?;
    scheduler.add(new_logfile_job).await?;

    Ok(scheduler)
}
