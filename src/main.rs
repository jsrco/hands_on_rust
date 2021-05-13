use std::io::stdin;

fn main() {
// got a new machine. just doing the entire book to relearn
    let mut your_name = String::new();
    println!("Hello, what is your name?");
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    println!("Hello, {}", your_name);
}
