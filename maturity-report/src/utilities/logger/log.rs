use std::{fs, path::Path};

use maturity_macro::maturity;
use tracing::info;
use tracing_subscriber::{
    EnvFilter, Layer,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::utilities::logger::result::LogResult;

#[maturity]
// This is taken from my other private crate.
pub fn initialise_logging() -> LogResult<()> {
    fs::create_dir_all("logs")?;

    archive_previous_log()?;

    let file = std::fs::File::create("./logs/latest.log")?;
    let filter = EnvFilter::new("trace")
        .add_directive("ignore=off".parse()?)
        .add_directive("globset=off".parse()?);
    let console_layer = fmt::layer().pretty().with_filter(filter.clone());

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(file)
        // .without_time()
        .compact()
        // .pretty()
        .with_level(true)
        .with_file(true)
        .with_line_number(false)
        .with_target(false)
        .with_filter(filter.clone());

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    info!("Initialised tracing logging successfully");

    Ok(())
}

#[maturity]
fn archive_previous_log() -> LogResult<()> {
    fs::create_dir_all("logs/archive")?;

    let latest = Path::new("logs/latest.log");

    if latest.exists() {
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S");
        // let timestamp = latest.metadata().unwrap().created().unwrap();
        let archived = format!("logs/archive/{}.log", timestamp);

        info!("Archived previous log to {}", archived);

        fs::rename(latest, archived)?;
    }
    Ok(())
}
