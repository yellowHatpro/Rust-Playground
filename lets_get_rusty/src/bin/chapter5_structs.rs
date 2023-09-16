#[derive(Debug)]
struct User {
    username: String, //String because we want each instance of this struct to own all of its data
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl blocks house methods and functions related to the struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//We can create multiple implementation blocks of a struct
impl Rectangle {
    //This is an associative function
    //A method contains self reference, but an associative function does not
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

//Tuple struct
struct Color(i32, i32, i32); // without named fields
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("yellowhatpro3119@gmail.com"),
        username: String::from("yellowhatpro"),
        sign_in_count: 12,
        active: true,
    };
    let _name = user1.username;
    user1.username = String::from("Ashu");

    let user2 = User {
        email: String::from("ashuaswal@gmail.com"),
        username: String::from("Ashutosh"),
        ..user1 // this will fill whatever field is left from user1
    };

    let rect = Rectangle {
        height: 32,
        width: 60,
    };

    let _rect2 = Rectangle::square(12);

    println!("The area of the rectangle is {} square pixels", rect.area());

    //Before printing a struct we need to do a couple of things before
    // println!("User : {}", user1); // ERROR: User struct does not implement Display Trait
    // Display trait specifies how something is printed, primitive types implement Display Trait
    // by default
    // This is how we do it in custom structs:
    //1. {:?} or {:#?}
    //2. #[derive(Debug)]
    println!("User: {:#?}", user1);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
