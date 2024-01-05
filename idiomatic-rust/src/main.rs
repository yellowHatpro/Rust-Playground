use idiomatic_rust::{Post, User};

trait Animal{
    fn speak(&self);
}
#[derive(Debug)]
struct Cat {
    name: String,
}

#[derive(Debug)]
struct Dog {
    name: String,
}

impl Animal for Cat{
    fn speak(&self) {
        println!("meow!");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof!");
    }
}

fn main() {
    //unwrap_or_default only works for types which implement Default trait
    let user1 = User::new("testuser123".to_owned()).unwrap_or_default();
    println!("{:?}", user1);

    let post1 = Post::default();
    println!("{:?}", post1); //Post { content: "", tags: [], likes: 0 }
    let post2 = Post::new("Lol".to_owned());

    let peanut = "peanut".to_owned();
    let oreo = "oreo".to_owned();

    print_animal_name(&oreo);

    let cat = Box::new(Cat { name: peanut });
    let dog = Box::new(Dog { name: oreo });

    print_dog(&dog);

    let animals: Vec<Box<dyn Animal>> = vec![cat, dog];
    animal_sounds(&animals);
}

fn print_animal_name(name: &String) {
    println!("{name}");
}

fn print_dog(dog: &Box<Dog>) {
    println!("{:?}", dog);
}

fn animal_sounds(animals: &Vec<Box<dyn Animal>>) {
    for a in animals {
        a.speak();
    }
}