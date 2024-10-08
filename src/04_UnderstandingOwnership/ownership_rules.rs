/*

The double colon :: operator allows us to namespace this particular
from function under the String type.



*/

fn main() {
    
    invalid_reference();

    let mut s = String::from("Hello");
    s.push_str(", world.!"); // push_str() appends a literal to a String
    println!("{s}");
}

fn invalid_reference() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world!"); // cannot borrow as mutable
}