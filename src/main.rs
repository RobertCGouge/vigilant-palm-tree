use sqlx::postgres::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_config;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_config().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect_lazy_with(configuration.database_configuration.connection_options());

    let address = format!(
        "{}:{}",
        configuration.application_configuration.host, configuration.application_configuration.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
