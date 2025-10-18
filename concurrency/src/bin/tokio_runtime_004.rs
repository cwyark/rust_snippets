/*
* create a single thread runtime using macro;
*/

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // everything run on the current thread;
    println!("single thread async.");
}
