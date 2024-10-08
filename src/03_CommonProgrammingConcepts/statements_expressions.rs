// Function bodies are made up of a series of statements optionally ending
// in an expression. 

fn main() {
    let y = 6; // statement (declaração) Statements do not return values.

    let x = {
        let z = 3;
        z + 1 // Expressions do not include ending semicolons.
    };

    println!("The value of x = {x}"); // 4

    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {number}");

}

/*

Statements are instructions that perform some action and do not return
a value.

Expressions evaluate to a resultant value.




*/