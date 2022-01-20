use std::env;

fn main() {
    let command = env::args()
        .nth(1)
        .expect("tempconvert must be called with at least 1 argument");

    // Show help and exit
    if &command == "--help" || &command == "-h" {
        println!("Help:");

        return;
    }

    let value_to_convert = env::args()
        .nth(2)
        .expect("tempconvert must be provided a value to convert");

    let parsed_value: f32 = value_to_convert
        .trim()
        .parse()
        .expect("tempconvert must be provided a valid number");

    let result: f32 = match command.as_str() {
        "-f" | "--fahrenheit" => convert_to_fahrenheit(parsed_value),
        "-c" | "--celsius" => convert_to_celsius(parsed_value),
        _ => 0.0,
    };

    println!("Result: {}", result);
}

fn convert_to_fahrenheit(val: f32) -> f32 {
    val * 1.8 + 32.0
}

fn convert_to_celsius(val: f32) -> f32 {
    (val - 32.0) / 1.8
}
