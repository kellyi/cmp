fn nth_word(s: &String, nth: u32) -> &str {
    let bytes = s.as_bytes();

    let mut number_of_spaces_to_find = nth - 1;
    let mut init = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if number_of_spaces_to_find == 0 {
                return &s[init..i];
            } else {
                number_of_spaces_to_find = number_of_spaces_to_find - 1;
                init = i + 1;
            }
        }
    }

    &s[init..]
}

fn first_word(s: &String) -> &str {
    nth_word(s, 1)
}

fn second_word(s: &String) -> &str {
    nth_word(s, 2)
}

fn third_word(s: &String) -> &str {
    nth_word(s, 3)
}

fn main() {
    let s = String::from("hello world");

    let first = first_word(&s);

    println!("{}", first);

    let second = second_word(&s);

    println!("{}", second);

    let t = String::from("get third word");

    let third = third_word(&t);

    println!("{}", third);
}
