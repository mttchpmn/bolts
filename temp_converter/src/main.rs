use std::io;

fn main() {
    println!("-----| Temp converter |-----\n");

    println!("Select conversion type:");
    println!("    F) Convert to Fahrenheit");
    println!("    C) Convert to Celsius");

    let conversion_type = get_user_input("Enter your selection:");
    let conversion_type: String = match conversion_type.trim() {
        "F" => String::from("fahrenheit"),
        "C" => String::from("celsius"),
        _ => String::from("celsius"),
    };

    let temp_value = get_user_input("Enter the temperature value to convert:");
    let temp_value: f32 = temp_value.trim().parse().expect("Failed to convert number");

    let result: f32 = match conversion_type.as_str() {
        "celsius" => convert_to_celsius(temp_value),
        "fahrenheit" => convert_to_fahrenheit(temp_value),
        _ => convert_to_celsius(temp_value),
    };

    println!("Converted temp value: {}", result);
}

fn get_user_input(msg: &str) -> String {
    println!("{}", msg);
    let mut result = String::new();

    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");

    result
}

fn convert_to_fahrenheit(val: f32) -> f32 {
    val * 1.8 + 32.0
}

fn convert_to_celsius(val: f32) -> f32 {
    (val - 32.0) / 1.8
}
