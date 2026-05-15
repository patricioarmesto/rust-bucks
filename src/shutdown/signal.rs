use tracing::info;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    ctrl_c.await;

    info!("shutdown signal received");
}
