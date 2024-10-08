/*

If you have loops within loops, break and continue apply to the innermost
loop at that point.

Loop labels must begin with a single quote.

*/

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    let mut index = 0;
    
    while index < 5 { // another languages: while (index < 5)
        println!("the value is: {}", a[index]);
        index += 1;
    }

// Using the for loop, you wouldn’t need to remember to change any other
// code if you changed the number of values in the array

    for element in a {
        println!("the value is: {element}");
    }
}
