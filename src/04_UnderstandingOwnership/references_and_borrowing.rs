/*

These ampersands represent references, and they
allow you to refer to some value without taking ownership of it.

The opposite of referencing by using & is dereferencing, which is
accomplished with the dereference operator, *.

The &s1 syntax lets us create a reference that refers to the value
of s1 but does not own it.

Because it does not own it, the value it points to will not be
dropped when the reference stops being used.

Likewise, the signature of the function uses & to indicate that
the type of the parameter s is a reference.

*/

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // So, what happens if we try to modify something we’re borrowing?
    // Spoiler alert: it doesn’t work!
    let s = String::from("hello");
    change(&s);
}

// We call the action of creating a reference borrowing.
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, the String is not dropped.

fn change(some_string: &String) {
    some_string.push_str(", world"); // Attempting to modify a borrowed value
}
