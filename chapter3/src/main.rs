fn main() {
    // FIBONACCI
    /* 
    const NUMBER: i32 = 4;
    let valor = fibonacci(NUMBER);
    println!("fibonnaci 4 = {} ", valor)
    */

    // fahrenheit to celcius
    println!("{} ", fahrenheit_to_celcius(100.0))
}

fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 0.5556
}

fn fibonacci(value: i32) -> i32 {
    if value == 1 { value } else {value * fibonacci(value - 1)}
}