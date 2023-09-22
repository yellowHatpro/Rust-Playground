pub mod generics {

    struct Point<T> {
        x: T,
        y: T,
    }

    struct Cursor<T, U> {
        x: T,
        y: U,
    }

    impl<U> Point<U> {
        pub fn x(&self) -> &T {
            &self.x
        }
    }

    pub fn generics() {
        let _p1 = Point { x: 5, y: 6 };
        let _c1 = Cursor { x: 4, y: 3.0 };
    }

    pub fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
        let mut largest = number_list[0];
        for num in number_list {
            if num > largest {
                largest = num;
            }
        }
        largest
    }
    //PartialOrd is a trait : We are restricting the type valid for T to only those that implement
    //PartialOrd, which enables comparisons
}
