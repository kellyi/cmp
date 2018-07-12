extern crate clap;

use std::collections::HashMap;
use clap::App;

// Given a list of integers, use a vector and return the mean (the average
// value), median (when sorted, the value in the middle position), and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.

fn convert_input_to_vector(input: &str) -> Vec<i32> {
    input
        .trim_matches('[')
        .trim_matches(']')
        .split(',')
        .map(|e| {
            let i: i32 = e
                .parse()
                .expect("Unable to read argument as integer");

            i
        })
        .collect()
}

fn print_mean(numbers: &Vec<i32>) {
    let total: i32 = numbers
        .iter()
        .sum();

    let mean = total as f32 / numbers.len() as f32;

    println!("Mean of {:?} is {}", numbers, mean);
}

fn print_median(numbers: &Vec<i32>) {
    let mut sorted = numbers.clone();
    sorted.sort();

    if let Some(median) = sorted.get(numbers.len() / 2) {
       println!("Mean of {:?} is {}", numbers, median);
    }
}

fn print_mode(numbers: &Vec<i32>) {
    let mode_dict: HashMap<i32, i32> = numbers
        .iter()
        .fold(HashMap::new(), |mut dict, &x| {
            {
                *dict.entry(x).or_insert(0) += 1;
            }
            dict
        });

    if let Some(max_occurences) = mode_dict.values().max() {
        if let Some(first) = numbers.get(0) {
            let mode = numbers
                .iter()
                .fold(first, |acc, x| {
                    match mode_dict.get(x) {
                        Some(v) if v == max_occurences => x,
                        _ => acc,
                    }
                });

            println!("Mode of {:?} is {}", numbers, mode);
        }
    }
}

fn main() {
    let matches = App::new("List stats")
        .version("1.0")
        .about("Get stats about a list of integers")
        .args_from_usage("-l, --list=[LIST] 'List of integers formatted like [-1,0,1,2,3]'")
        .get_matches();

    match matches.value_of("list") {
        Some(n) => {
            let numbers = convert_input_to_vector(n);

            print_mean(&numbers);
            print_median(&numbers);
            print_mode(&numbers);
        }
        _ => println!("None!"),
    }
}
