fn main() {
    
    let s = String::from("hello");
    let slice_a = &s[0..2];
    let slice_b = &s[..2]; // if you want to start at index 0, you can drop
    // the value before the two periods.
    let slice_c = &s[3..];
    let slice_d = &s[..]; //You can also drop both values to take a slice of the entire string. 
}