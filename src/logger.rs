use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::{
    EnvFilter, Layer, Registry, fmt, layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[allow(dead_code)]
pub struct LogGuards {
    pub app_guard: WorkerGuard,
    pub api_guard: WorkerGuard,
    pub stdout_guard: WorkerGuard,
}
pub fn init_logging(log_level: &str) -> LogGuards {
    // App logs
    // let app_file = rolling::daily("logs", "app.log");
    // let (app_writer, _guard1) = tracing_appender::non_blocking(app_file);

    // let app_layer = fmt::layer()
    //     .with_writer(app_writer)
    //     .with_ansi(false)
    //     .with_target(true)
    //     .with_level(true)
    //     .with_filter(
    //         EnvFilter::try_new(app_log_level)
    //             .unwrap_or_else(|_| EnvFilter::new("info")),
    //     );

    // // API logs (will be handled separately via tower_http)
    // let api_file = rolling::daily("logs", "api.log");
    // let (api_writer, _guard2) = tracing_appender::non_blocking(api_file);

    // let api_layer = fmt::layer()
    //     .with_writer(api_writer)
    //     .with_ansi(false)
    //     .with_target(false)
    //     .with_level(true)
    //     .with_filter(
    //         EnvFilter::try_new("info")
    //             .unwrap_or_else(|_| EnvFilter::new("info")),
    //     );

    // // Compose the subscriber
    // let subscriber = Registry::default().with(app_layer).with(api_layer);

    // tracing::subscriber::set_global_default(subscriber)
    //     .expect("Failed to set global tracing subscriber");

    // let app_file = rolling::daily("logs", "app.log");
    // let (app_writer, app_guard) = non_blocking(app_file);

    // let api_file = rolling::daily("logs", "api.log");
    // let (api_writer, api_guard) = non_blocking(api_file);

    // // Only logs NOT targeting "api"
    // let app_layer = fmt::layer()
    //     .with_writer(app_writer)
    //     .with_filter(filter_fn(|metadata| metadata.target() != "api"));

    // let api_layer = fmt::layer()
    //     .with_writer(api_writer)
    //     .with_filter(filter_fn(|metadata| metadata.target() == "api"));

    // Console appender (stdout)
    let (stdout_writer, _stdout_guard) = non_blocking(std::io::stdout());
    // Console layer
    let console_layer = fmt::Layer::default()
        .with_writer(stdout_writer.with_max_level(tracing::Level::INFO))
        .with_ansi(true) // pretty color output
        .with_target(false)
        // .compact()
        .with_filter(
            EnvFilter::try_new(log_level)
                .unwrap_or_else(|_| EnvFilter::new("info")),
        );

    Registry::default()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        // .with(app_layer)
        // .with(api_layer)
        .with(console_layer)
        .init();

    // Return the guards to keep the writers alive
    LogGuards {
        // app_guard: app_guard,
        // api_guard: api_guard,
        app_guard: _stdout_guard,
        api_guard: tracing_appender::non_blocking(std::io::sink()).1,
        stdout_guard: tracing_appender::non_blocking(std::io::sink()).1,
    }
}
