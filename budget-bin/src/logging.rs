use std::fs;
use std::path::Path;

pub fn setup_logging(log_path: &Path) {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(
            fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(log_path)
                .unwrap(),
        )
        .apply()
        .unwrap();

    log::info!("starting logging, log file: {:?}", log_path);
}
