extern crate clap;

use clap::App;

fn convert_to_celsius(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn convert_to_fahrenheit(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn main() {
    let matches = App::new("Convert temperature")
        .version("1.0")
        .about("Convert temperature from Celsius to Fahrenheit and vice versa")
        .args_from_usage("-f, --fahrenheit=[FAHRENHEIT] 'Convert from Fahrenheit to Celsius'")
        .args_from_usage("-c, --celsius=[CELSIUS] 'Convert from Celsius to Fahrenheit'")
        .get_matches();

    let warning = "Integer argument required";

    if let Some(f) = matches.value_of("fahrenheit") {
        let fahr: f32 = f
            .parse()
            .expect(warning);

        let celsius = convert_to_celsius(fahr);

        println!("{}", celsius);
    } else if let Some(c) = matches.value_of("celsius") {
        let celsius: f32 = c
            .parse()
            .expect(warning);

        let fahr = convert_to_fahrenheit(celsius);

        println!("{}", fahr);
    }
}
