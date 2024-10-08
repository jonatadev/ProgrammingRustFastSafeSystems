fn main() {

    // First we change s to be mut.
    let mut s = String::from("hello");
    change(&mut s);
}

// Update the function signature to accept a mutable reference 
// with some_string: &mut String. 
// This makes it very clear that the change function will mutate
// the value it borrows.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
