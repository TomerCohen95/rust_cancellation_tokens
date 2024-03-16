use tokio::{signal::ctrl_c, time::{sleep, Duration}};
use tokio_util::sync::CancellationToken;
use std::sync::Arc;
use std::future::Future;

async fn configuration_updates_task(task_id: u32) {
    println!("Configuration updates task {} started.", task_id);
    sleep(Duration::from_secs(30)).await;
    println!("Configuration updates task {} completed.", task_id);
}

async fn resource_metrics_report_task(task_id: u32) {
    println!("Resource metrics report task {} started.", task_id);
    sleep(Duration::from_secs(30)).await;
    println!("Resource metrics report task {} completed.", task_id);
}

async fn scan_files_task(task_id: u32) {
    println!("Scan files task {} started.", task_id);
    sleep(Duration::from_secs(30)).await;
    println!("Scan files task {} completed.", task_id);
}

mod orchestrator {
    use tokio::task::{JoinHandle, spawn};
    use tokio_util::sync::CancellationToken;
    use std::sync::Arc;
    use std::pin::Pin;

    pub async fn orchestrate_tasks(tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>) {
        let cancellation_token = Arc::new(CancellationToken::new());

        let spawned_tasks: Vec<_> = tasks.into_iter()
            .map(|task| spawn(task))
            .collect();

        tokio::select! {
            _ = cancel_on_ctrl_c(Arc::clone(&cancellation_token)) => {
                println!("Cancellation task completed.");
            }
            _ = async {
                for task in spawned_tasks {
                    let _ = task.await;
                }
                println!("All tasks completed.");
            } => {
                println!("All Done.");
            }
        }
    }

    async fn cancel_on_ctrl_c(cancellation_token: Arc<CancellationToken>) {
        tokio::signal::ctrl_c().await.unwrap();
        println!("Cancellation signal sent.");
        cancellation_token.cancel();
    }
}

#[tokio::main]
async fn main() {
    let tasks = vec![
        Box::pin(configuration_updates_task(1)),
        Box::pin(resource_metrics_report_task(2)),
        Box::pin(scan_files_task(3)),
    ];

    orchestrator::orchestrate_tasks(tasks).await;
}
