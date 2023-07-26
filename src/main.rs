/**
 * Given temperature in degrees_celcius:
 * 1. Compute degrees_fahrenheit using the formula: degrees_fahrenheit = degrees_celcius * 1.8000 + 32.00
 * 
 * Given temperature in degrees_fahrenheit:
 * 1. Compute degrees_celcius using the formula: degrees_celcius = (degrees_fahrenheit - 32.00) / 1.8000
 */

 fn main() {
    convert_to_fahrenheit(-273.15);

    convert_to_celcius(-459.7)
}

// Given temperature in Celsius
fn convert_to_fahrenheit(degrees_celsius: f32) {
    let degrees_fahrenheit = degrees_celsius * 1.8000 + 32.00;

    println!("The temperature in Degrees Fahrenheit is: {}", degrees_fahrenheit);
}

fn convert_to_celcius(degrees_fahrenheit: f32) {
    let degrees_celcius = (degrees_fahrenheit - 32.00) / 1.8000;

    println!("The temperature in Degrees Celcius is: {}", degrees_celcius)
}