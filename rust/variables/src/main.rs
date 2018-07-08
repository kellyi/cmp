fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("{}", x);

    let spaces = "   ";

    let spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not real");

    println!("{}", spaces);
    println!("{}", guess);

    let i = 0xff;

    println!("{}", i);

    let j = 2.0;
    let k: f32 = 3.0;

    println!("{} {}", j, k);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("{} {} {} {} {}", sum, difference, product, quotient, remainder);

    let t = true;
    let f: bool = false;

    println!("{} {}", t, f);

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_, second, _) = tup;

    println!("{}", second);

    println!("{}", tup.2);

    let a = [1, 2, 3, 4, 5];
    let index = 5;
    println!("{}", a[index]);
}
