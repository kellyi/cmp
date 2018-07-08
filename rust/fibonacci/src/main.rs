extern crate clap;
use clap::App;

fn fibo(n: u64, acc: u64) -> u64 {
    match n {
        65 => panic!("Integer overflow"),
        0 => acc,
        _ => fibo(n - 1, acc * n),
    }
}

fn main() {
    let matches = App::new("Fibonacci")
        .version("1.0")
        .about("Generate nth Fibonacci number")
        .args_from_usage("-n, --nth=[NTH] 'Fibonacci number to generate'")
        .get_matches();

    let warning = "Positive integer argument required";

    if let Some(n) = matches.value_of("nth") {
        let nth: u64 = n
            .parse()
            .expect(warning);

        let nth = fibo(nth, 1);

        println!("{}th Fibonacci number is {}", n, nth);
    };
}
