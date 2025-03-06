fn main() {
    android_logger::init_once(android_logger::Config::default().with_tag("log_test"));
    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");
}
