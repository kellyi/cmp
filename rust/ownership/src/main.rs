fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(i: i32) {
    println!("{}", i);
}

fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

    println!("{} {}", s1, s3);

    let s4 = String::from("hello");

    let (s5, len) = calculate_length(s4);

    println!("{} {}", s5, len);
}
