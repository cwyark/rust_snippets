use log::{info, trace, warn};

// to run this program, use "RUST_LOG=trace ./simple_logger"
fn main() {
    env_logger::init();
    trace!("this is a trace log !");
    info!("this is a info log !");
    warn!("this is a warn log !");
}
