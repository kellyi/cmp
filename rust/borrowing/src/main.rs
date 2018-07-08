fn calculate_length(s: &String) -> usize {
    s.len()
}

fn mutate(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("length of {} is {}", s1, len);

    let mut s2 = String::from("hello");
    mutate(&mut s2);

    println!("{}", s2);

    let reference_to_something = no_dangle();

    println!("{}", reference_to_something);
}
