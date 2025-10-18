/*
* create a single thread runtime using manually creation.
*/

use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_current_thread()
        .enable_all() // enable timer and I/O driver.
        .build()
        .unwrap();

    rt.block_on(async {
        println!("running on the current thread.");
    });
}
