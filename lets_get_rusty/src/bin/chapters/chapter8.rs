pub mod strings {

    //Strings are stored as a collection of UTF-8 encoded bytes

    fn string_operations() {
        let mut s = String::from("foo");
        s.push("bar"); //can push string slice
        s.push('!'); //can push character
        let s1 = String::from("hello ");
        let s2 = String::from("world!");
        let s3 = format!("{}{}", s1, s2); // format macro does not take ownership of input params
    }

    pub fn strings() {
        let s1 = String::new();
        let s2 = "initial contents";
        let s3 = s2.to_string();
        let s4 = String::from("initial contents");
    }
}

pub mod vectors {

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn vector() {
        let a = [1, 2, 3];
        let mut v: Vec<i32> = Vec::new();
        v.push(1);
        v.push(2);

        //Create vector and initialise with values at the same time,we can use vec! macro
        let mut v2 = vec![1, 2, 3];
        let third = &v[2];
        match v2.get(2) {
            Some(i) => println!("The third element is {}", i),
            None => println!("Gracefully deal with no third element found"),
        }

        for i in &v2 {
            println!("{}", i);
        }

        for i in &mut v2 {
            *i += 50;
        }

        //Storing enums in vectors
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(12.1),
            SpreadsheetCell::Text(String::from("blue")),
        ];
    }
}
