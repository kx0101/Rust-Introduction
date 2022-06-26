fn main() {
    println!("Hello, world!");
    test_one();
    add_numbers(20, 30); // 50

    let number = {
        let x = 3;
        x + 1 // we're not adding a semicolon here but at the very end so we can return the x + 1.
        // we would get an error without a semicolon because we wouldn't return anything as a value to number variable.
    };
    println!("{}", number); // number is a statement and inside of it we have an expression. The inside is an expression because
    // it returns a value and then we assign it to number variable.

    let result = add_numbers_second(2, 3);
    println!("{}", result); // 5

}

// snake case
fn test_one() {
    println!("Test has been called..");
}

// specifying the type of the parameters.
fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

// With the arrow we're specifying the type of the return.
fn add_numbers_second(x: i32, y: i32) -> i32 {
    x + y // if we added a semicolon it wouldn't work for the reason explained above.   

    // or

    return x + y; // we can use semicolon with return.
}

// if we don't return anything with the function then we don't have to specify the return type.