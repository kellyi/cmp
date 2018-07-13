use std::thread;
use std::time::Duration;

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calcluating slowly!");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "today do {} pushups",
            expensive_result.value(intensity),
        );

        println!(
            "next do {} situps",
            expensive_result.value(intensity),
        );
    } else {
        if random_number == 1 {
            println!("take a break today. remember to stay hydrated");
        } else {
            println!(
                "today run for {} minutes",
                expensive_result.value(intensity),
            );
        }
    }
}

fn main() {
    let simulated_user_specific_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specific_value,
        simulated_random_number,
    );

    let x = 4;

    let equal_to_x = |z| z == x;
    let y = 4;

    // fn equal_to_x_(z: i32) -> bool { z == x }

    assert!(equal_to_x(y));
    // assert!(equal_to_x_(y));

    // let x = vec![1, 2, 3];

    // let equal_to_x_ = move |z| z == x;

    // println!("can't use x here {:?}", x);

    // let y = vec![1, 2, 3];

    // assert!(equal_to_x_(y));

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{}", val);
    }

    let v2 = vec![1, 2, 3];

    let mut v2_iter = v2.iter();

    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    let v3: Vec<i32> = vec![1, 2, 3];
    let v4: Vec<i32> = v3.iter().map(|x| x + 1).collect();

    assert_eq!(v4, vec![2, 3, 4]);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        match self.count < 6 {
            true => Some(self.count),
            _ => None,
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    assert_eq!(18, sum);
}
