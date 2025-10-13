use std::thread::{self, sleep};
fn main() {
    println!("Start program here !");
    let t1 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(200));
        println!("the long running task finish last!");
    });

    let t2 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(100));
        println!("we can chain callbacks..");
        let t3 = thread::spawn(move || {
            sleep(std::time::Duration::from_millis(50));
            println!("like this..");
        });
        t3.join().unwrap();
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
