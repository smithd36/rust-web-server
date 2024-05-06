/**
 * Author: ds
 * Date: 2021
 * A basic rust web server tracing crate for middleware logging.
 */
use tracing::{info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

fn setup_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    set_global_default(subscriber).expect("setting default subscriber failed");
}

fn main() {
    setup_logging();
    info!("Server starting up");
    // Server startup logic...
}