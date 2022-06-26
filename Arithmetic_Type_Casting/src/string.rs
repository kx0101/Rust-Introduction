use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read the line");

    let int_input: i64 = input.trim().parse().unwrap(); // we're trimming in order to remove the \n 

    println!("{}", int_input + 2);
}