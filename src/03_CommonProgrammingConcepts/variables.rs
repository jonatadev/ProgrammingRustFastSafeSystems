
fn main() {

    let mut x = 5; // mut, faz com a variável seja mutavel
    println!("The value of x: {x}");
    x = 6; // cannot assign twice to immutable variable
    println!("The value of x: {x}");

    // Constants are immutable by default
    // . Just know that you must always annotate the type. : u32
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}