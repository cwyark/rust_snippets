/*
* create a multi-thread runtime using macro.
*/

#[tokio::main]
async fn main() {
    // runs on a multi-thread runtime.
    tokio::spawn(async {
        println!("hello from a spawn task.");
    })
    .await
    .unwrap();
}
