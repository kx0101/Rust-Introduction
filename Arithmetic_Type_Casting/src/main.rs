fn main() {
    let x: u8 = 9; // 0 to 255
    let y: i8 = 10; // -128 to -127

    let z = x + y;
    println!("{}", z); // error because we cannot sum different types (u8 + i8) even though it is 8 bits.


    let x: u8 = 255;
    let y: u8 = 1;

    let z = x + y; // 266 not in range of 8 bits which is 0...255 since z is also type u8
    println!("{}", z); // error because of overflow.

    // Treat the number as the following type 

    let x = 200i8;
    // or
    let x = 200 as i8;
    // or
    let x = 200_i8;

    // floats

    let x = 200.0f32;
    let x = 200.0f64;


    // Example -- You wanna convert the smaller values to the larger as opposed to the other way around.

    let x = 127_000 as i64;
    let y= 10_i32;

    let z = x / (y as i64); // casting y to use i64 type
    println!("{}", z);


}
