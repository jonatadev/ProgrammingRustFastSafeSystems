fn main() {

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // This is called destructuring because it breaks the
    // single tuple into three parts.
    
    let (x, y, z) = tup;
    println!("The value of y is: {x}"); // 500

    // We can also access a tuple element directly by using a period (.) 
    // followed by the index of the value we want to access. 
 
    let get_one = tup.2;
    println!("The value of get_one is: {get_one}"); // 1 

    // Array Type
    // arrays in Rust have a fixed length
    // Arrays are useful when you want your data allocated on 
    // the stack rather than the heap
    // arrays are more useful when you know the number of elements will 
    // not need to change.  
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // Here, i32 is the type of each element. After the semicolon, the number 5
   //  indicates the array contains five elements.
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let az = [3; 5];
    let january = months[0];
    println!("First month of the year: {january}")
    

    



}