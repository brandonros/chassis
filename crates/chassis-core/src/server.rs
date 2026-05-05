use std::future::IntoFuture;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::shutdown::shutdown_signal;

pub async fn run(
    app_bind_addr: &str,
    app_router: Router,
    admin_bind_addr: &str,
    admin_router: Router,
) -> anyhow::Result<()> {
    let app_listener = TcpListener::bind(app_bind_addr)
        .await
        .with_context(|| format!("failed to bind app listener to {app_bind_addr}"))?;
    info!(addr = %app_listener.local_addr()?, "app listening");

    let admin_listener = TcpListener::bind(admin_bind_addr)
        .await
        .with_context(|| format!("failed to bind admin listener to {admin_bind_addr}"))?;
    info!(addr = %admin_listener.local_addr()?, "admin listening");

    let token = CancellationToken::new();

    {
        let token = token.clone();
        tokio::spawn(async move {
            shutdown_signal().await;
            token.cancel();
        });
    }

    let app_serve = axum::serve(app_listener, app_router)
        .with_graceful_shutdown(token.clone().cancelled_owned());
    let admin_serve =
        axum::serve(admin_listener, admin_router).with_graceful_shutdown(token.cancelled_owned());

    tokio::try_join!(app_serve.into_future(), admin_serve.into_future()).context("server error")?;

    Ok(())
}
