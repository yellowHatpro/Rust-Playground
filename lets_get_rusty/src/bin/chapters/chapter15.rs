pub mod box_smart_pointer {
    //
    // In many cases smart pointers own data they point to, unlike references which simply borrow
    // the values
    // Ex: Strings, Vectors are smart pointers
    // Smart pointers are implemented using structs
    // THey implement deref and drop traits unlike other structs
    // DEREF: allows the instances of the smart pointer structs to be treated as references
    // DROP : allows to customise the code when the instance of the smart pointer goes out of scope

    use std::{ops::Deref, rc::Rc};

    //BOX: allows allcate values in the heap
    pub fn box_smart_pointer() {
        let b = Box::new(5);
        println!("b= {}", b);
    }

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    //Deref trait allows pointers to be treated as normal references
    fn deref_example() {
        let x = 5;
        let y = Box::new(5);
        assert_eq!(5, *y);
    }

    //Build own Smart pointer
    //My box_smart_pointer
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    //Deref coersion:
    fn hello(name: &str) {
        println!("Hello, {}", name);
    }

    fn main() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);
        hello(&(*m)[..]); // if rust didn't do deref coersion
    }
    //We passed &MyBox<String> --- on deref -> &String -- on deref -> &str (as String also
    //implements deref trait)

    // Deref coersion doesn't work while going from immutable reference to mutable reference, this
    // has to be done by borrowing rules

    //DROP trait :
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data {}", self.data);
        }
    }

    //Rc (Reference count (in single threaded systems))
    enum ListRc {
        Cons(i32, Rc<ListRc>), //Cons own the data they hold
        Nil,
    }

    //Reference count lets 2 variables point to same memory reference,
    //Rc allows multiple parts of data to read the same data , not modify it
    // Rc::clone -> increases the reference count
    fn lear_reference_count() {
        let a = Rc::new(ListRc::Cons(
            10,
            Rc::new(ListRc::Cons(4, Rc::new(ListRc::Nil))),
        ));
        let b = ListRc::Cons(3, Rc::clone(&a));
        let c = ListRc::Cons(3, Rc::clone(&a));
    }
}
