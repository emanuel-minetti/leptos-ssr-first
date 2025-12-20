use log::{log, Level};
use sqlx::{query, Pool, Postgres};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

/// To be used as background task
async fn session_cleanup_task(db_pool: Pool<Postgres>) {
    let query_result = query!(
        r#"
                    DELETE FROM session
                    WHERE expires_at < CURRENT_TIMESTAMP - INTERVAL '60 minutes';
                    "#
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

pub async fn setup_scheduler(db_pool: Pool<Postgres>) -> Result<JobScheduler, JobSchedulerError> {
    let scheduler = JobScheduler::new().await?;
    let db_pool = db_pool.clone();
    scheduler.start().await?;
    // TODO: set to half an hour after testing
    let session_cleanup_job = Job::new_async("0 1/5 * * * *", move |_uuid, _l| {
        let db_pool = db_pool.clone();

        Box::pin(async move {
            session_cleanup_task(db_pool).await;
        })
    })?;
    scheduler.add(session_cleanup_job).await?;

    Ok(scheduler)
}
