fn main() {
    println!("{}", fahrenheit_to_celsius(65.0));
    println!("{}", celsius_to_fahrenheit(21.0));
    println!("{}", fib(9));
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    1.8 * c + 32.0
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
