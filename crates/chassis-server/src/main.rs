#[tokio::main]
async fn main() -> anyhow::Result<()> {
    chassis_core::run().await
}
