extern crate clap;
use clap::App;

fn one() {
    let result: i32 = (1..1000)
        .map(|n| {
            match n {
                n if n % 5 == 0 => n,
                n if n % 3 == 0 => n,
                _ => 0,
            }
        })
        .sum();

    println!("{}", result);
}

fn main() {
    let matches = App::new("PE")
        .version("1.0")
        .about("Solving problems")
        .args_from_usage("-n, --nth=[NTH] 'Problem to solve'")
        .get_matches();

    match matches.value_of("nth") {
        Some("1") => one(),
        _ => println!("None"),
    }
}
