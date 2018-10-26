use amethyst::{StdoutLog, LoggerConfig, LogLevelFilter};

use log::debug;

use std::io;

pub fn start_logger(config: LoggerConfig) {
    let mut dispatch = basic_dispatch(config.level_filter);

    match config.stdout {
        StdoutLog::Plain => dispatch = dispatch.chain(io::stdout()),
        StdoutLog::Colored => dispatch = dispatch.chain(colored_stdout()),
        StdoutLog::Off => {}
    }

    if let Some(path) = config.log_file {
        match fern::log_file(path) {
            Ok(log_file) => dispatch = dispatch.chain(log_file),
            Err(_) => eprintln!("Unable to access the log file, as such it will not be used"),
        }
    }

    dispatch = dispatch.level_for("gfx_device_gl::factory", LogLevelFilter::Warn);

    dispatch.apply().unwrap_or_else(|_| {
        debug!("Global logger already set")
    });
}

fn basic_dispatch(level_filter: LogLevelFilter) -> fern::Dispatch {
    fern::Dispatch::new()
        .level(level_filter)
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{level}][{target}] {message}",
                level = record.level(),
                target = record.target(),
                message = message,
            ))
        })
}

fn colored_stdout() -> fern::Dispatch {
    let color_config = fern::colors::ColoredLevelConfig::new();

    fern::Dispatch::new()
        .chain(io::stdout())
        .format(move |out, message, record| {
            let color = color_config.get_color(&record.level());
            out.finish(format_args!(
                "{color}{message}{color_reset}",
                color = format!("\x1B[{}m", color.to_fg_str()),
                message = message,
                color_reset = "\x1B[0m",
            ))
        })
}
