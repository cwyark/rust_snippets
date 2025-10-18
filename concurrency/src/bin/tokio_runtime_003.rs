/*
* create a runtime using Runtime::new
*/
use tokio::runtime::Runtime;
fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        println!("run on a runtime.");
    });
}
