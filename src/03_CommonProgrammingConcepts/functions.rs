// We define a function in Rust by entering fn followed by a function
// name and a set of parentheses. 

fn main(){
    another_function(100);
    let five = five();
    println!("five() = {five}");
}

// In function signatures, you must declare the type of each parameter.
fn another_function(x: i32) {
    println!("Another function!, x value {x}");
}

// The body of the function is a lonely 5 with no semicolon
// because it’s an expression whose value we want to return.

fn five() -> i32 {
    5
}