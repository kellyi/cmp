fn main() {
    println!("Hello, world!");

    another_function();
    second_function(5);
    third_function(5, 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);

    let x = fourth_function();

    println!("{}", x);

    let z = plus_one(5);

    println!("{}", z);
}

fn another_function() {
    println!("Another function!");
}

fn second_function(x: i32) {
    println!("x is {}", x);
}

fn third_function(x: i32, y: i32) {
    println!("{}", x * y);
}

fn fourth_function() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
