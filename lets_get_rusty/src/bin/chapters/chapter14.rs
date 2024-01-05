pub mod iterators {
    //All iterators in rust implement the iterator trait, which is defined in rust standard library

    pub trait Iterator {
        type Item; //associated type , more in chapter 19
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    pub fn main() {
        let v1 = vec![1, 2, 3];
        //In rust iterators are lazy, nothing happens when we declare the following line, until we
        //use the value
        let v1_iter = v1.iter();
        for value in v1_iter {
            println!("Got: {}", value);
        }
    }

    pub fn adapter_method() {
        let v1 = vec![1, 2, 3];
        //Remember in rust iterators are lazy, therefore we need a consumer method collect to
        //collect the result
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    }
}
