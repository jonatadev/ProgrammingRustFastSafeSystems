/*

A struct’s name should describe the significance of the pieces of data
being grouped together.

Then, inside curly brackets, we define the names and types of the pieces of data,
which we call fields.

To use a struct after we’ve defined it, we create an instance of that struct by
specifying concrete values for each of the fields.

We don’t have to specify the fields in the same order in which we declared them in the struct.

*/

// Rust also supports structs that look similar to tuples, called tuple structs.

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {

    let jonatas = User {
        active: true,
        username: String::from("Jonatas Alves da Silva"),
        email: String::from("jonatadev@gmail.com"),
        sign_in_count: 1,
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    // To get a specific value from a struct, we use dot notation.
    // If the instance is mutable, we can change a value by using the dot notation and
    // assigning into a particular field.
    jonatas.email = String::from("jonatasdevbackup@gmail.com");

    // Rust doesn’t allow us to mark only certain fields as mutable.

    // The syntax .. specifies that the remaining fields not explicitly 
    // set should have the same value as the fields in the given instance.
    let user2 = User {
        email: String::from("jonatasrust@gmail.com"),
        ..jonatas // Struct Update Syntax,  The .. user1 must come last 
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // Using the Field Init Shorthand
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
