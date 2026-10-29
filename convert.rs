use std::io;
use std::io;

fn main() {
    println!("Temperature Convert!");
    println!("Enter a mode:");
    println!("[1] Celsius -> Fahrenheit");
    println!("[2] Fahrenheit -> Celsius");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mode: i32 = input.trim().parse().expect("Please enter 1 or 2");

    println!("Enter the temperature value:");
    let mut temp_input = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read line");

    let x: f64 = temp_input.trim().parse().expect("Please enter a valid number");

    if mode == 1 {
        // Celsius → Fahrenheit
        let fahrenheit = x * 9.0 / 5.0 + 32.0;
        println!("{x} °C = {fahrenheit} °F");
    } else if mode == 2 {
        // Fahrenheit → Celsius
        let celsius = (x - 32.0) * 5.0 / 9.0;
        println!("{x} °F = {celsius} °C");
    } else {
        println!("Invalid mode. Please choose 1 or 2.");
    }
}