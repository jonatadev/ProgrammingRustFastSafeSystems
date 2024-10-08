/*

If we tried to use s after the call to takes_ownership, Rust would
throw a compile-time error. These static checks protect us from mistakes. 

*/

fn main() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s);

    // Tentar usar o s aqui!
    println!("s value = {s}"); //  ERROR: value borrowed here after move

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
