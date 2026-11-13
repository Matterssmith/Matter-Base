mod commands;
mod core;
mod endpoints;
mod providers;
mod router;

#[tokio::main]
async fn main() {
    if let Err(error) = core::bootstrap::run().await {
        eprintln!("✖ Matter Base no pudo iniciar: {error:#}");
        std::process::exit(1);
    }
}
