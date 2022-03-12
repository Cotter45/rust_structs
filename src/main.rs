// create a struct a lot like interface in typescript
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32); // even with same types
struct Point(i32, i32, i32); // these are completely different structs 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangle method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

}

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
    user1.username = String::from("anotherusername");
    user1.active = false;
    user1.sign_in_count = 2;

    // create a new user a reuse previously built structs data
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // creating tuple structs
    // access with ie - black.1 / origin.2 (index as key)
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // project here t0 find the area of a rectangle
    // let width1 = 30;
    // let height1 = 50;
    // let rect1 = (30, 50);
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);



}

// factory function to create new users
fn _build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


