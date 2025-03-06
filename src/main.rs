fn main() {
    android_logger::init_once(android_logger::Config::default().with_tag("log_test").with_max_level(log::LevelFilter::Info));
    // Without this, log crate will apply its own filter regardless of platform overrides
    log::set_max_level(log::LevelFilter::Trace);
    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");
}
