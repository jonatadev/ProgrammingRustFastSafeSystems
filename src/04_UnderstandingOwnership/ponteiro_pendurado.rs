/*

In Rust, by contrast, the compiler guarantees that references will never be
dangling references.

if you have a reference to some data, the compiler will ensure that the data will
not go out of scope before the reference to the data does.

*/

fn main() {
    let reference_to_nothing = dangle(); 
}

fn dangle() -> &String {
    // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!

/*
Because s is created inside dangle, when the code of dangle is finished, s
will be deallocated. But we tried to return a reference to it. That means this
reference would be pointing to an invalid String. That’s no good! Rust won’t
let us do this. */

fn no_dangle() -> String { // The solution here is to return the String directly
    let s = String::from("hello");
    s
} //  Ownership is moved out, and nothing is deallocated.
