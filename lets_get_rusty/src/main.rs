fn main() {
    let s = String::from("Ashu");
    checking_integers_borrow(s.clone());
    println!("{}", s);
}

fn checking_integers_borrow(a: String) {
    println!("{}", a);
}
