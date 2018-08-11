extern crate libc;

use std::env;
use std::process;
use std::str::FromStr;
use std::ffi::CStr;

// adapted from https://github.com/fengcen/hostname/blob/master/src/lib.rs
extern "C" {
    fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
}

pub fn get_hostname() -> Option<String> {
    let mut buffer = Vec::<u8>::with_capacity(255);
    let pointer = buffer.as_mut_ptr() as *mut libc::c_char;

    unsafe {
        match gethostname(pointer, buffer.capacity() as libc::size_t) {
            0 => {
                let hostname = CStr::from_ptr(pointer)
                    .to_string_lossy()
                    .into_owned();

                Some(hostname)
            },
            _ => None,
        }
    }
}

enum DisplayLength {
    Full,
    Short,
}

impl FromStr for DisplayLength {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "-s" | "--short" => Ok(DisplayLength::Short),
            "-f" | "--full" => Ok(DisplayLength::Full),
            _ => Err(()),
        }
    }
}

struct Arguments {
    _program_name: String,
    display_length: DisplayLength,
}

impl Arguments {
    pub fn create(args: Vec<String>) -> Arguments {
        let (program_name, flags) = args.split_at(1);

        let _program_name = program_name
            .iter()
            .map(|v| String::from(v.clone()))
            .collect();

        let flags: Vec<String> = flags
            .iter()
            .map(|v| String::from(v.clone()))
            .collect();

        let display_length = match flags.get(0) {
            Some(arg) => arg
                .parse()
                .unwrap_or(DisplayLength::Full),
            None => DisplayLength::Full,
        };

        Arguments {
            _program_name,
            display_length,
        }
    }
}

fn main() {
    let args = Arguments::create(env::args().collect());

    match (get_hostname(), args.display_length) {
        (Some(h), DisplayLength::Full) => println!("{}", h),
        (Some(h), DisplayLength::Short) => {
            let elements: Vec<String> = h
                .split(".")
                .map(|v| String::from(v.clone()))
                .take(1)
                .collect();

            match elements.get(0) {
                Some(s) => println!("{}", s),
                None => process::exit(1),
            };
        },
        _ => println!("None"),
    };
}
