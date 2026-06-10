use std::thread;
use std::sync::Arc;

fn main() {
    let name = Arc::new(String::from("Dioman"));
    let name2 = Arc::clone(&name);
    thread::spawn(move || {
        print!("{}", name2);
    });

    println!("{}", name);
}
