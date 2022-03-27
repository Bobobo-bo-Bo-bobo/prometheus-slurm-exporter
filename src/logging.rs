pub fn init(level: log::LevelFilter) -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|logout, logmsg, logrecord| {
            logout.finish(format_args!(
                "{:<6}: {} {}",
                logrecord.level(),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%z"),
                logmsg
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}
