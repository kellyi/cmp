use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

#[derive(Debug)]
enum AnotherList {
    AnotherCons(Rc<RefCell<i32>>, Rc<AnotherList>),
    AnotherNil,
}

use AnotherList::{AnotherCons, AnotherNil};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping MyBox value");
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let boxy = Box::new(5);
    println!("boxy = {}", boxy);

    let _list = Cons(1,
        Rc::new(Cons(2,
            Rc::new(Cons(3,
                Rc::new(Nil))))));


    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));

    hello(&m);
    drop(y);
    hello(&(*m)[..]);

    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));

    let b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
    println!("reference count for a is {}", Rc::strong_count(&a));

    {
        let _d = Cons(5, Rc::clone(&a));
        println!("refernce count for a is {}", Rc::strong_count(&a));
    }

    println!("reference count for a is {}", Rc::strong_count(&a));
    drop(b);
    println!("reference count for a is {}", Rc::strong_count(&a));

    //     let n = 5;
    //     let o = &mut n;

    let new_value = Rc::new(RefCell::new(5));

    let new_a = Rc::new(AnotherCons(Rc::clone(&new_value), Rc::new(AnotherNil)));

    let new_b = AnotherCons(Rc::new(RefCell::new(6)), Rc::clone(&new_a));
    let new_c = AnotherCons(Rc::new(RefCell::new(10)), Rc::clone(&new_a));

    *new_value.borrow_mut() += 10;

    println!("new_a after {:?}", new_a);
    println!("new_b after {:?}", new_b);
    println!("new_c after {:?}", new_c);
}
