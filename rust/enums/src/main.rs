enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_type: &IpAddrKind) {
    match ip_type {
        IpAddrKind::V4 => println!("V4"),
        _ => println!("V6"),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(&four);
    route(&six);

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("{}", n),
        _ => println!("None"),
    }

    match some_string {
        Some(s) => println!("{}", s),
        _ => println!("None"),
    }

    if let Some(n) = absent_number {
        println!("{}", n);
    } else {
        println!("None!");
    }
}
