use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your text: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("{}", input);
}
