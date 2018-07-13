fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    match x.len() > y.len() {
        true => x,
        _ => y,
    }
}

fn first_string_argument<'a>(x: &'a str, _: &str) -> &'a str {
    x
}

fn main() {
    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }

    //     println!("r: {}", r);
    // }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(
        string1.as_str(),
        string2,
    );

    println!("The longest string is {}", result);

    let string3 = String::from("very long string");
    {
        let string4 = String::from("xyz");
        let new_result = longest(
            string3.as_str(),
            string4.as_str(),
        );

        println!("The longest string is {}", new_result);
    }

    let string5 = String::from("first string");
    let string6 = String::from("second string");

    let first_string = first_string_argument(
        string5.as_str(),
        string6.as_str(),
    );

    println!("The first string is {}", first_string);
}
