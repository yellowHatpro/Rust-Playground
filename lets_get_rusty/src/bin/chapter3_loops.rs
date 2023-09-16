fn main() {
    let mut counter = 0;
    let returned_value = loop {
        counter += 1;
        if counter == 10 {
            break counter; // returns value out of the loop
        }
    };
    println!("counter value : {}", returned_value);
}

fn while_loop() {
    let mut number = 12;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFT OFF!");
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5];
    for e in a.iter() {
        println!("the value is: {}", e);
    }
    for number in 1..4 {
        println!("{}", number);
    }
}
