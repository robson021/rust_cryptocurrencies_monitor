use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::console::Target;
use log4rs::config::{Appender, Config, Root};

pub fn setup_logger() {
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    let error_trace_level = Root::builder().appender("stderr").build(LevelFilter::Trace);

    let config = Config::builder()
        .appender(Appender::builder().build("stderr", Box::new(stderr)))
        .build(error_trace_level)
        .unwrap();

    log4rs::init_config(config).unwrap();
    log::set_max_level(LevelFilter::Info);
}
