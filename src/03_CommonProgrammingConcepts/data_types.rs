fn main() {
    //  The compiler can usually infer what type we want to use based on the value and how we use it.
    let guess: u32 = "42".parse().expect("Not a number!");
    // A scalar type represents a single value.

    // floating point
    let x = 2.0;
    let y: f32 = 3.0; // f32

    let t = true;
    let f: bool = false; // with explicit type annotation
    
    // Note that we specify char literals with single quotes, as opposed to
    // string literals, which use double quotes.
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation

}