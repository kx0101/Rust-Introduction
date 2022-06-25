fn main() {
    println!("Hello, world!");

    let x: i32 = 2; // Can be both negative and positive

    // Bits
    // i8
    // i16
    // i32
    // i64
    // i128

    let x: u32 = 2; // Can't be negative

    // u8 0 - 2^8 - 1 (Numbers that you can have as your input) - (0 - 255)
    // i8 -2^7 - 2^7 - 1 (-128 - 127)

    let floating_point: f32 = 10.9;

    let true_or_false = false;

    let letter: char = 'a';


    let tup: (i32, bool, char) = (1, true, 'c');
    let tup2: (i8, bool, char) = (1, true, 'c');

    // The above two have different types

    // In order to print a tup element you need to specify the index of the tup. For example:
    println!("{}", tup); // error
    println!("{}", tup.1); // will print 'true'


    // Change a tup value 

    let mut tup3: (i32, bool, char) = (1, true, 'c');

    tup3.0 = 2;

    // You cannot change the type of the tuple. E.g.
    let tup = (1, true, 'c', 10); //error


    
    let mut arr: [i32, 5];
    println!("{}", arr[4]) // error because we haven't initialized all the elements of the array

    let mut arr2: [i32, 5] = [];
    println!("{}", arr[4]) // error because the array is empty

    let mut arr3: [i32, 5] = [1, 2, 3, 4, 5];
    // or
    let mut arr4 = [1, 2, 3, 4, 5];
    
    // ways of initializing an array
    
    
    // extra tip

    let x: u8 = 4;
    let y = x;

    // y is the same type as x so that means u8 and if we try to do something like that:

    let y: i32 = x; // this will occur an error


}

// Scalar Types


// Integers

// let x: i32 means assigning x as a 32bit integer (unsigned int in C++).
// let x: u32 means assigning x as a positive integer. 


// Floats

// let floating_point: f32 means assigning floating_point as a single precision float. Whereas f64 means double precision.
// The default is f64.


// Boolean

// let true_or_false: bool means assigning true_or_false as a boolean.


// Chars

// let letter: char = 'a' means assigning letter as a char with SINGLE quotes.



// Tuples

// Immutable but we can use 'mut' in order to make them mutable.



// Arrays

// Immutable but we acn use 'mut' in order to make them mutable.