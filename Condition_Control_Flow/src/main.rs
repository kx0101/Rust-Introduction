fn main() {
    let cond = 2 < 3;
    println!("{}", cond); // true

    let cond = 2 < 2.2; // error: We need to use the same types or convert one of the two for example

    let cond = (2 as f32) < 2.2; // true

    // Compound

    // AND
    let cond2 = true && cond; // true

    // OR
    let cond2 = false && cond; // true

    // NOT
    let cond2 = !(false && cond); // false

    // Which one is first ?
    // Answer: ! && ||


    // if else
    let food = "cookie";

    if (food == "cookie") {
        println!("I like cookies too!");
    } else if (food == "fruit") {
        println!("That sounds healthy!");
    } else {
        println!("Oh, that's too bad!");
    }

}
