pub mod lifetimes {
    use std::fmt::Display;

    //aka Generic Lifetime Annotations
    // LIFETIME RULES:
    // 1. Each parameter that is a reference gets its own lifetime parameter
    // 2. If there is exactly 1 input param, that lifetime is assigned to all output lifetime
    //    params
    //    3. If there are >1 lifetime params, but one of them is &self or &mut self the lifetime of
    //       self is assigned to all output lifetime params. (3rd one applies to methds)
    // we can add the lifetime annotations, but the condition is satisfied due to rule 3
    impl<'a> ImportantExcerpt<'a> {
        fn return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    // pub fn dangling_reference() {
    //let r;
    //  {
    //let x = 5;
    //r = &x; // x does not live long enough for us to print r down there
    //}
    //r here is a dangling reference
    //   println!("r: {}", r);
    //}

    pub fn string_example() {
        let string1 = String::from("abcd");
        let string2 = String::from("xyz");
        let res = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", res);
    }

    // &i32 -> reference
    // &'a i32 -> reference with an explicit lifetime
    // &'a mut i32 -> mutable reference with an explicit lifetime

    //we dont know the lifetime of x or y here, fix : use lifetimes (<'anything>)
    // the lifetime of the return type of the function needs to be one of the lifetime passed as
    // input
    pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // we can return String (own datatypes and not references) type and not &str as it transfers ownership
    pub fn longest_example<'a>(x: &str, y: &str) -> String {
        let res = String::from("some answer but it works");
        res
    }

    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    //Static Lifetime
    // The reference can live as long as the duration of the program
    // all string literals have static lifetime : because they are stored in program's binary

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("announcement {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    pub fn main() {
        let novel = String::from("Call me ashu. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find anything");
        // the lifetime of i is tied to first_sentence, lives as long as first_sentence
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}
