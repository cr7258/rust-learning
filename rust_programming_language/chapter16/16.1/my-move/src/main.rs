use std::thread;

fn main() {
    let v = vec![1,2,3];

    let handler = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handler.join().unwrap();
}
