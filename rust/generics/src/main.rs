fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    list
        .iter()
        .fold(list[0], |lg, &next| match next > lg {
            true => next,
            _ => lg,
        })
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("{}", largest(&number_list));

    let negative_number_list = vec![-1, -2, -3, -4, -5];

    println!("{}", largest(&negative_number_list));

    let char_list = vec!['z', 'b', 'e', 'f'];

    println!("{}", largest(&char_list));
}
