/*
* create a multi-thread runtime using manually creation.
*/

use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread()
        .worker_threads(4) // set number of workers threads (default = $cpus)
        .enable_all() // enable time and I/O driver.
        .build()
        .unwrap();

    rt.block_on(async {
        tokio::spawn(async {
            println!("running on worker pool");
        })
        .await
        .unwrap();
    });
}
