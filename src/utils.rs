use std::io::Write;

pub fn init_logger() {
    let log_filters = std::env::var("RUST_LOG").unwrap_or_default();

    env_logger::Builder::new()
        .parse_filters(&log_filters)
        .format(|formatter, record| {
            writeln!(
                formatter,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
}
