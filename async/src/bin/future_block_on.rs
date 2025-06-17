use futures::executor::block_on;

async fn do_work() {
    println!("hello, async world!");
}

fn main() {
    println!("hello, sync world");
    block_on(do_work());
}
