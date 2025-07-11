mod evm_job;

use anyhow::Context;
use database::{AppService, DatabasePool};
use evm::EVMClient;
use std::{sync::Arc, time::Duration};
use tokio::sync::{Mutex, Notify};
use tokio_cron_scheduler::{Job, JobScheduler};
use utils::env::Env;

pub async fn run() -> Result<(), anyhow::Error> {
    let env = Env::init();
    let connection = DatabasePool::init(&env)
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));
    let db = Arc::new(connection);
    let service = Arc::new(AppService::init(&db, &env));
    let evm_client = Arc::new(EVMClient::init(
        &env.dao_contract_address,
        &env.wallet_private_key,
        &env.rpc_url,
        env.chain_id,
    ));

    let is_evm_job_running = Arc::new(Mutex::new(false));
    let job_completed_notify = Arc::new(Notify::new());

    let mut scheduler = serve(
        service,
        evm_client,
        env,
        is_evm_job_running.clone(),
        job_completed_notify.clone(),
    )
    .await?;

    // Handle shutdown signals (SIGTERM from systemctl)
    #[cfg(unix)]
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to install signal handler")
                .recv()
                .await;
        } => {},
    };

    #[cfg(not(unix))]
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = async {
            tokio::signal::windows::ctrl_break()
                .expect("failed to install CTRL_BREAK handler")
                .recv()
                .await;
        } => {},
    };

    // Shutdown logic
    let running = is_evm_job_running.lock().await;
    if *running {
        println!("Waiting for current job to complete (max 5 mins)...");
        drop(running); // Release lock before waiting

        // Wait for either job completion or timeout
        tokio::select! {
            _ = job_completed_notify.notified() => {
                println!("Job completed - shutting down");
            },
            _ = tokio::time::sleep(Duration::from_secs(5 * 60)) => {
                println!("Timeout reached - forcing shutdown");
            }
        }
    } else {
        println!("No job running - shutting down immediately");
    }
    scheduler.shutdown().await?;

    Ok(())
}

pub async fn serve(
    service: Arc<AppService>,
    evm_client: Arc<EVMClient>,
    env: Env,
    is_evm_job_running: Arc<Mutex<bool>>,
    job_completed_notify: Arc<Notify>,
) -> anyhow::Result<JobScheduler> {
    // Initialize and start the scheduler
    let scheduler = JobScheduler::new()
        .await
        .context("Failed to create job scheduler")?;

    let job_service = service.clone();
    let job_env = env.clone();
    let job_is_running = is_evm_job_running.clone();
    let schedule = env.evm_job_schedule.clone();
    let job_evm_client = evm_client.clone();

    scheduler
        .add(
            Job::new_async(&schedule, move |_uuid, _l| {
                println!("evm job run: {}", env.now());
                let service = job_service.clone();
                let env = job_env.clone();
                let running_flag = job_is_running.clone();
                let evm_client = job_evm_client.clone();
                let completion_notify = job_completed_notify.clone();

                Box::pin(async move {
                    let mut running = running_flag.lock().await;
                    if *running == false {
                        *running = true;
                        drop(running);

                        let result = evm_job::run(service, evm_client, env).await;

                        if let Err(err) = result {
                            println!("evm job failed: {:?}", err);
                        }

                        let mut running = running_flag.lock().await;
                        *running = false;
                        completion_notify.notify_one();
                    } else {
                        println!("Skipping - job already running");
                    }
                })
            })
            .context("Failed to create evm job")?,
        )
        .await
        .context("Failed to add evm job to scheduler")?;

    scheduler
        .start()
        .await
        .context("Failed to start scheduler")?;

    Ok(scheduler)
}
