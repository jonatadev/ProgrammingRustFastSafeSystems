/*

we don’t have a call to clone, but x is still valid and wasn’t moved into y.

The reason is that types such as integers that have a known size at compile 
time are stored entirely on the stack, so copies of the actual values are quick
to make. 

There’s no difference between deep and shallow copying here.

*/
fn main() {

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}
