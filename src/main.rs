/**
 * Given temperature in degrees_celcius:
 * 1. Compute degrees_fahrenheit using the formula: degrees_fahrenheit = degrees_celcius * 1.8000 + 32.00
 */

 fn main() {
    convert_to_fahrenheit(-273.15)
}

// Given temperature in Celsius
fn convert_to_fahrenheit(degrees_celsius: f32) {
    let degrees_fahrenheit = degrees_celsius * 1.8000 + 32.00;

    println!("The temperature in Degrees Fahrenheit is: {}", degrees_fahrenheit);
}
