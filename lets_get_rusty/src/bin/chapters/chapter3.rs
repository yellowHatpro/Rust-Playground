pub mod basics {
    fn print_area(x: i32, y: i32) {
        println!("Area of the rectangle is {}", x * y);
    }

    //last expression is returned if return is not specified, don't use semi colon in this case
    fn return_area_of_circle(r: i32) -> f32 {
        3.14 * r * r
    }
    pub fn basics() {
        //variables
        let x = 5;
        println!("The value of x is {}", x);
        let x = "six";
        println!("The new value after shadowing of x is {}", x);

        // Consts in rust need to be constants, & they can't be something that needs to be calculated on runtime
        const CONSTANT_IN_RUST: u32 = 1000_000; // consts in rust needs to be typed
        let tup = ("Let's get rusty", 100_00);
        //destructuring a tuple
        let (project, fees) = tup;
        let fees = tup.1; //access 2nd element of tuple

        //Arrays in Rust are fixed size, for dynamic types use Vectors
        let error_codes = [200, 404, 500];
        print_area(20, 20);

        let area_of_circle = return_area_of_circle(12);
        if area_of_circle < 10 {
            println!("small circle ");
        } else {
            println!("big circle");
        }
    }
}

pub mod loops {
    pub fn loops() {
        let mut counter = 0;
        let returned_value = loop {
            counter += 1;
            if counter == 10 {
                break counter; // returns value out of the loop
            }
        };
        println!("counter value : {}", returned_value);
    }
    pub fn while_loop() {
        let mut number = 12;
        while number != 0 {
            println!("{}", number);
            number -= 1;
        }
        println!("LIFT OFF!");
    }

    pub fn for_loop() {
        let a = [1, 2, 3, 4, 5];
        for e in a.iter() {
            println!("the value is: {}", e);
        }
        for number in 1..4 {
            println!("{}", number);
        }
    }
}
