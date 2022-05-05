use std::thread;

fn main() {
    let handlle = thread::spawn(|| {
        println!("Hello, world!");
    });
    dbg!(handlle.join());
}
