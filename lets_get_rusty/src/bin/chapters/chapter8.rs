pub mod strings {
    //Strings are stored as a collection of UTF-8 encoded bytes

    fn string_operations() {
        let mut s = String::from("foo");
        s.push_str("bar"); //can push string slice
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

    pub fn different_encodings() {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    pub fn indexing_into_strings() {
        let s1 = String::from("hello");
        //let h = s1[0]; // this is wrong, String cannot be indexed by integer
        //3 relevant ways to look at strings from Rust's perspective: Bytes, Scaler values, and Grapheme clusters
        let hello = String::from("नमस्ते");
        //Bytes
        //[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
        for b in hello.bytes() {
            println!("{}", b);
        }

        //Scaler values
        //['न', 'म', 'स', '्', 'त', 'े']
        for c in hello.chars() {
            println!("{}", c);
        }

        // Grapheme clusters
        // ["न", "म", "स्", "ते"]
        // the ability to iterate over Grapheme clusters is not included by default
        // import unicode-segmentation crate
        use unicode_segmentation::UnicodeSegmentation;
        for g in hello.grapheme_indices(true) {
            println!("{:#?}", g);
        }
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

pub mod hashmaps {
    use std::collections::HashMap;
    pub fn hashmaps() {
        let blue = String::from("Blue");
        let yellow = String::from("Yellow");

        let mut scores = HashMap::new();
        scores.insert(blue, 10); // ownership moved
        scores.insert(yellow, 50); //ownership moved
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        for (key, value) in &scores {
            println!("{}:{}", key, value);
        }

        let mut scores_mut = HashMap::new();
        // if no entry then insert 30, otherwise not
        scores_mut.entry(String::from("Yellow")).or_insert(30);

        // Updating value based on old value
        // or_insert returns a mutable reference to the value, which we can dereference using *
        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
