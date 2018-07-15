fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("{}, {}", x, y);
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("favorite color is {}", color);
    } else if is_tuesday {
        println!("is tuesday");
    } else if let Ok(age) = age {
        println!("{}", age);
    } else {
        println!("no matches");
    }

    let mut stack = Vec::new();

    (1..4)
        .for_each(|i| {
            stack.push(i);
        });

    while let Some(element) = stack.pop() {
        println!("{}", element);
    }

    let point = (3, 5);

    print_coordinates(&point);

    // if let x = 5 {
    //     println!("{}", x);
    // }

    let x = 10;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        n if n % 2 == 0 => println!("even"),
        _ => println!("anything"),
    }

    let s = Some(5);

    match s {
        Some(5) => println!("five"),
        Some(v) => println!("{}", v),
        None => println!("none"),
    }

    let z = 1;

    match z {
        1 | 2 => println!("one or two"),
        _ => println!("anything"),
    }

    let r = 5;

    match r {
        1 ... 10 => println!("one through ten"),
        _ => println!("something else"),
    }

    let c = 'c';

    match c {
        'a' ... 'j' => println!("early ascii letter"),
        'k' ... 'z' => println!("late ascii letter"),
        _ => println!("something else"),
    }

    let p = Point {
        x: 0,
        y: 7,
    };

    let Point { x: lng, y: lat } = p;

    assert_eq!(0, lng);
    assert_eq!(7, lat);

    let other_p = Point {
        x: 0,
        y: 7,
    };

    let Point { x, y } = other_p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match other_p {
        Point { x: _, y: 0 } => println!("y is 0"),
        Point { x: 0, y: _ } => println!("x is 0"),
        Point { x, y } => println!("x, y -> {}, {}", x, y),
    }

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    fn foo(_: i32, y: i32) {
        println!("{}", y);
    }

    foo(3, 4);

    let numbers = (2, 4, 6, 8, 10);

    match numbers {
        (first, _, third, _, _) => {
            println!("{}, {}", first, third);
        }
    }

    if let Some(_) = Some(String::from("Hello")) {
        println!("Some");
    }

    let origin = Point { x: 0, y: 2 };

    match origin {
        Point { x, .. } => println!("{}", x),
    }

    let second = (1, 2, 3, 4, 5);

    match second {
        (first, ..) => {
            println!("{}", first);
        }
    }

    let robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref name) => println!("{}", name),
        _ => (),
    }

    println!("{:?}", robot_name);

    let mut another_name = Some(String::from("Bors"));

    match another_name {
        Some(ref mut name) => *name = String::from("Another name"),
        _ => (),
    }

    println!("{:?}", another_name);

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than 5"),
        Some(x) => println!("{}", x),
        _ => (),
    }

    enum Message {
        Hello {
            id: i32,
        },
    }

    let msg = Message::Hello {
        id: 5,
    };

    match msg {
        Message::Hello {
            id: id_variable @ 3...7,
        } => {
            println!("in range {}", id_variable);
        },
        Message::Hello {
            id: id_variable @ 10...12,
        } => {
            println!("in another range {}", id_variable);
        },
        Message::Hello { id } => println!("{}", id),
    }
}
