use std::io;

fn main() {
    println!("Enter an input");

    let mut input = String::new();
    // I'm calling the new function from the io library. We're using :: operators for that.

    io::stdin().read_line(&mut input).expect("Failed to read line"); // expect is basically catching errors 
    // allowing us to access the input variable and actually input something
    println!("{}", input);


    let mut input = 4; // error because we're waiting to read a String
}


// In order to use input we have to use the 'io' library (input/output)
// The syntaxx is 
// use std::'name';
