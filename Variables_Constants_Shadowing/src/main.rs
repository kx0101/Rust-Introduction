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
}

// You can either say what type the variable is or let the compiler do it instead.

// By default in rust all variables are immutable which means they cannot change.

// We can fix it by adding 'mut' before the variable name.