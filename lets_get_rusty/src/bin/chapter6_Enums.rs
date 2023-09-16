enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_func() {
        println!("Lesgo");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
}

fn route(ip_kind: IpAddrKind) {}

fn option_enum() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; //for none we need to annotate types
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => None, // _ for all other cases, although with Options only 2 enums are there
    }
}
