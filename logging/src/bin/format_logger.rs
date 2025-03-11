use log::{info, trace, warn};
use std::io::Write;

// to run this program, use "MY_LOG_LEVEL=trace ./simple_logger"
fn main() {
    let env = env_logger::Env::default()
        .filter("MY_LOG_LEVEL")
        .write_style("MY_LOG_STYLE");

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    trace!("this is a trace log !");
    info!("this is a info log !");
    warn!("this is a warn log !");
}
