use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    let other_v = vec![1, 2, 3];

    v.push(1);
    v.push(2);
    v.push(3);

    let first: &i32 = &v[0];
    let second: &i32 = &other_v[1];

    println!("{}", first);
    println!("{}", second);

    let third: Option<&i32> = other_v.get(3);

    match third {
        Some(n) => println!("{}", n),
        _ => println!("None"),
    }

    if let Some(n) = v.get(2) {
        println!("{}", n);
    }

    let another_vec = vec![100, 32, 57];

    for i in &another_vec {
        println!("{}", i);
    }

    let mut yet_another_vec = vec![100, 32, 57];
    for i in &mut yet_another_vec {
        *i += 50;
    }

    for i in &yet_another_vec {
        println!("{}", i);
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let data = "initial contents";

    let s = String::from("initial contents");

    let z = "initial_contents".to_string();

    println!("{} {} {}", data, s, z);

    let strings = vec![
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שָׁלוֹם"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    for e in strings {
        println!("{}", e);
    }

    let mut st = String::from("foo");
    st.push_str(" bar");

    println!("{}", st);

    let mut s1 = String::from("foo");
    let s2 = " bar baz";
    s1.push_str(s2);

    println!("s2 is {}", s1);

    let mut s3 = String::from("lo");
    s3.push('l');

    println!("{}", s3);

    let s4 = String::from("Hello ");
    let s5 = String::from("world");
    let s6 = s4 + &s5;

    println!("{}", s6);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let tic_tac_toe = format!("{}-{}-{}", tic, tac, toe);

    println!("{}", tic_tac_toe);

    let hello = String::from("hello");
    match hello.chars().nth(0) {
        Some(h) => println!("{}", h),
        _ => println!("None"),
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    if let Some(score) = scores.get(&team_name) {
        println!("{}", score);
    }

    for (key, value) in &scores {
        println!("{} -> {}", key, value);
    }

    scores.insert(String::from("Blue"), 50);
    scores.entry(String::from("Red")).or_insert(77);

    println!("{:?}", scores);

    let teams = vec![
        String::from("Blue"),
        String::from("Yellow"),
    ];

    let initial_scores = vec![10, 50];

    let _zipped_scores: HashMap<_, _> = teams
        .iter()
        .zip(initial_scores.iter())
        .collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let text = "hello world wonderful world";

    let mut another_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = another_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", another_map);
}
