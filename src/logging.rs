pub fn init_logging() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Trace)
        .init();
}
