// create a struct a lot like interface in typescript
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32); // even with same types
struct Point(i32, i32, i32); // these are completely different structs 

fn main() {

    // to use a struct we must create a new instance
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // access and change with . notation like JS as long as its mutable
    user1.email = String::from("anotheremail@example.com");

    // create a new user a reuse previously built structs data
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };



}

// factory function to create new users
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


