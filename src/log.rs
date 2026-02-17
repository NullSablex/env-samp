pub fn init_logging() -> bool {
    print_banner();
    true
}

pub fn info(msg: &str) {
    log::info!("{}", msg);
}

pub fn warn(msg: &str) {
    log::warn!("{}", msg);
}

pub fn error(msg: &str) {
    log::error!("{}", msg);
}

fn print_banner() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let repository = env!("CARGO_PKG_REPOSITORY");
    let build_date = env!("BUILD_DATE");
    let build_time = env!("BUILD_TIME");
    let build_year = env!("BUILD_YEAR");

    log::info!("");
    log::info!("  | {} {} | {}", name, version, build_year);
    log::info!("  |-------------------------------");
    log::info!("  | Author and maintainer: {}", value_or(author, "Unknown"));
    log::info!("");
    log::info!("  | Compiled: {} at {}", build_date, build_time);
    log::info!("  |-------------------------------");
    log::info!("  | Repository: {}", value_or(repository, "N/A"));
    log::info!("");
}

fn value_or<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.trim().is_empty() {
        fallback
    } else {
        value
    }
}
