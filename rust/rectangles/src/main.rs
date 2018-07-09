fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2((width, height): (u32, u32)) -> u32 {
    width * height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn hello(&self) {
        println!("hello!");
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn add_one(n: &i64) -> i64 {
        n + 1
    }
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} sq pixels",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} sq pixels",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} sq pixels",
        area3(&rect2)
    );

    println!("rect2 is {:?}", rect2);

    println!(
        "The area of rect2 is {} sq pixels",
        rect2.area()
    );

    rect2.hello();

    let rect3 = Rectangle { width: 30, height: 50 };
    let rect4 = Rectangle { width: 10, height: 40 };
    let rect5 = Rectangle { width: 60, height: 45 };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    let sq = Rectangle::square(3);

    println!(
        "the area of sq is {}",
        sq.area()
    );

    let z = 5;

    println!("{}", Rectangle::add_one(&z));
}
