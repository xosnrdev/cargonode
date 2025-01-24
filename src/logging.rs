use env_logger::Builder;
use log::LevelFilter;

pub fn get_logging(verbosity: u8) -> Builder {
    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let mut builder = env_logger::Builder::new();
    builder.filter(None, level);
    builder.format_module_path(false);

    if level == LevelFilter::Trace || level == LevelFilter::Debug {
        builder.format_timestamp_secs();
    } else {
        builder.format_target(false);
        builder.format_timestamp(None);
    }
    builder
}
