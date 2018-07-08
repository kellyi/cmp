fn main() {
    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    let condition = true;

    let another_number = if condition {
        5
    } else {
        6
    };

    println!("{}", another_number);

    let mut mutable_number = 3;

    while mutable_number != 0 {
        println!("{}", mutable_number);

        mutable_number = mutable_number - 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("element is {}", element);
    }

    for n in (1..4).rev() {
        println!("{}", n);
    }
}
