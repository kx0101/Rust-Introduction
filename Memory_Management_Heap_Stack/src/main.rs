fn main() {
    let x = 2;
}

// Stack 

// Last In First Out (LIFO).
// fastest prio to use stack rather than heap.

// Address Name Value
//   0      x    2
//   1
//   2
//   3
//   4

// After the execution of a function Rust will remove the x variable from the RAM.

// We can't store a dynamic value to the stack because all of these values are only capable of being deleted or added.
// We can store only something with a fixed size. (e.g. true, 2, 1.5 etc.)

fn main_second() {
    let x = 2;
    let y = x;

    example();
}

fn example() {
    let a = "true";
    let b = false;
}

// Address Name Value
//   0      x    2
//   1      y    2
//   2      a  "true"
//   3      b   false
//   4

// Firstly, we store the x and y value to the Stack. Then, we call the example function which adds to the stack the a and b values.
// After the example function (scope) is done, the stack will remove the a and b BUT NOT the x, y.
// This is happening because after the example function is over we'll go back to the point we called the example function.
// Which is on line 26 and as we can see we're not yet at the end of the main_second scope.
// Afterwards, example will be done and so will the main_second so everything in Stack will be removed.

fn main_third() {
    let x = 2;
    let y = x;

    add(x, y);
}

fn add(x: i32, y: i32) {
    x + y
}

// Address Name Value
//   0      x    2
//   1      y    2
//   2      x    2
//   3      y    2
//   4

// The function add is basically looking for the most recent x and y values that are in the Stack.





// Heap

// We're searching through the heap and look for a location that's large enough to hold what it is that we want to store.
// much more time consuming
// Mapping values to an address and then we use that address to access the value from the stack.

fn main_forth() {
    let x = 2;
    let string = String::from("hello"); // dynamic value
}

// stack

// Address Name Value
//   0      x    2
//   1    string (points to the address of value in the heap and then it gets the value from the heap) 
//   2          
//   3          
//   4

// heap

// Address Name Value
//   5          hello    
//   6          
//   7          
//   8          
//   9