use std::io::Error;
use std::fmt;

type MyResult<T> = Result<T, Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> MyResult<usize>;
    fn flush(&mut self) -> MyResult<()>;

    fn write_all(&mut self, buf: &[u8]) -> MyResult<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> MyResult<()>;
}

type Kilometers = i32;

type Thunk = Box<Fn() + Send + 'static>;
fn _takes_long_type(_f: Thunk) {}
// fn returns_long_type() -> Thunk {}

fn _bar() -> ! {
    loop {
        print!("loop forever");
    }
}

fn main() {
    let _f: Thunk = Box::new(|| println!("hi"));
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y == {}", x + y);

//     _bar();
}
