use std::sync::Arc;
use tokio::{signal::ctrl_c, time::{sleep, Duration}};
use tokio_util::sync::CancellationToken;

async fn configuration_updates_task(task_id: u32) {
    println!("Configuration updates task {} started.", task_id);
    sleep(Duration::from_secs(30)).await;
    println!("Configuration updates task {} completed.", task_id);
}

async fn resource_metrics_report_task(task_id: u32, cancellation_token: Arc<CancellationToken>) {
    println!("Resource metrics report task {} started.", task_id);

    tokio::select! {
        _ = cancellation_token.cancelled() => {
            println!("Resource metrics report task {} was canceled.", task_id);
            return;
        }
        _ = async {
            sleep(Duration::from_secs(30)).await;
            println!("Resource metrics report task {} completed.", task_id);
        } => {}
    }
}

async fn scan_files_task(task_id: u32) {
    println!("Scan files task {} started.", task_id);
    sleep(Duration::from_secs(30)).await;
    println!("Scan files task {} completed.", task_id);
}

#[tokio::main]
async fn main() {
    let cancellation_token = Arc::new(CancellationToken::new());

    let tasks = vec![
        tokio::spawn(configuration_updates_task(1)),
        tokio::spawn(resource_metrics_report_task(2, cancellation_token.clone())),
        tokio::spawn(scan_files_task(3)),
    ];

    let cancellation_token_clone = Arc::clone(&cancellation_token);
    let cancellation_task = tokio::spawn(async move {
        ctrl_c().await.unwrap();
        println!("Cancellation signal sent.");
        cancellation_token_clone.cancel();
    });

    tokio::select! {
        _ = cancellation_task => {
            println!("Cancellation task completed.");
        }
        _ = async {
            for task in tasks {
                let _ = task.await;
            }
            println!("All tasks completed.");
        } => {println!("All Done.");}
    };
}