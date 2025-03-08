use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from the spawned thread!");
    });

    handle.join();
}
