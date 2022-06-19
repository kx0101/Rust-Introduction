fn main() {
    // let x = 4;
    // println!("x is: {}", x);

    // mutable example

    let mut x = 4;
    println!("x is: {}", x);
    x = 5;
    println!("x is: {}", x);

    // another way of dealing with immutability

    // let x = 4;
    // println!("x is: {}", x);
    // let x = 5;
    // println("x is: {}", x);
    

    // let x: u32 = 4;
    // Assigned as u32 integer

    // let x = 4;
    // let x = "hello";
    // this would work because we're re-assigning the x variable with a type of string.

    // ALTHOUGH
    // let mut x = 4;
    // x = hello;
    // this would NOT work as it is waiting for an integer and not a string AND we have assigned the variable x to be an integer.


    // CONST - cannot be changed AND cannot re-assigned
    // const SECONDS_IN_MINUTES: u32 = 60;

}

// You can either say what type the variable is or let the compiler do it instead.

// By default in rust all variables are immutable which means they cannot change.

// We can fix it by adding 'mut' before the variable name.