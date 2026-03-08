use std::io::{Result, stdout};
use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listner = TcpListener::bind(address)?;

    run(listner, connection_pool)?.await
}
