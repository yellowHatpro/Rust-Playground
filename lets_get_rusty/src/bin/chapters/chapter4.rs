pub mod dynamic_allocation {
    pub fn dynamic_allocation() {
        //fn a, b stores in stack memory
        fn _a() {
            let x = "hello";
            let y = 22;
            _b();
        }
        fn _b() {
            let x = String::from("world"); // this is stored in heap memory
            let v: Vec<i32> = Vec::new();
        }
    }
}
pub mod ownership {

    // Rules of ownership
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value is dropped.

    pub fn ownership() {
        let s = String::from("hello");
        takes_ownership(s);
        //println!("{}", s); //This will give error since the ownership of s is moved to the function
        //parameter some string, will not work in case of ints, since they are copied, not moved
        let s1 = gives_ownership(); // The ownership is moved from function to s1 variable
        let s3 = takes_ownership_and_gives_back(s1);
        println!("{}", s3);
    }

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }
    fn takes_ownership_and_gives_back(a_string: String) -> String {
        a_string
    }

    pub fn copy_move_dynamic_allocation() {
        //String literals are stored in binary and are fixed in size
        let s = "hello";
        //This is dynamic type string which is stored in heap.
        //Deallocation is done automatically
        let heap_s = String::from("hello");

        let s1 = String::from("hello");
        let s2 = s1; // s1 is now invalid, this is called a move, remember only 1 owner rule

        let s3 = s2.clone(); // Now move is not done here

        let x = 5;
        let y = x; //Copy, not move (works on simple types like integers, chars, booleans)
    }
}

pub mod references {

    //References do not take ownership of the values, they borrow
    //References are immutable by default
    //Rules of references
    //1. At any given time, you can have either one mutable reference or any number of immutable references
    //2. References must always be valid // should not be dangling reference

    pub fn references_and_borrowing() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("length of the string is : {}", len);
        let mut s2 = String::from("hello");
        transform_string(&mut s2);

        //We cannot create 2 mutable references of a particular data
        let mut s = String::from("hello");
        let r1 = &mut s;
        //let r2 = &mut s; //ERROR
        //println!("{},{}", r1, r2);

        //scope of references: starts from where the vars were initialised, and continues through the
        //last time that reference is used.
        let mut sx = String::from("hello");
        let r1 = &sx;
        let r2 = &sx;
        println!("{},{}", r1, r2);
        let r3 = &mut sx; // This is valid, since the scope of r1 and r2 ends after println
    }

    pub fn calculate_length(s: &String) -> usize {
        s.len()
    }

    pub fn transform_string(hello: &mut String) {
        hello.push_str(" world");
    }

    fn borrowing() {
        let s = String::from("Ashu");
        checking_integers_borrow(s.clone()); // cloned, and thus ownership is not taken
        borrow(&s); //borrowed the value of s here
        println!("{}", s); //therefore we can use this s here as well
    }

    fn checking_integers_borrow(a: String) {
        println!("{}", a);
    }
    fn borrow(s: &String) {
        println!("{}", s);
    }
}

pub mod slices {
    //Slices
    //Let us reference a contiguous sequence of elements within a collection, instead of referencing
    //entire collection
    pub fn slices() {
        let mut s = String::from("hello world");
        let hello = &s[..5]; //5 exclusive
        let world = &s[6..];
    }
}
