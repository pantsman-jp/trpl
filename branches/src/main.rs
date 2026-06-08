fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    fizz_buzz(9);
    let condition = true;
    let x = if condition { 3 } else { 4 };
    println!("The value of x is: {x}")
}

fn fizz_buzz(n: i32) {
    if n % 15 == 0 {
        println!("FizzBuzz");
    } else if n % 5 == 0 {
        println!("Buzz");
    } else if n % 3 == 0 {
        println!("Fizz");
    } else {
        println!("{n}")
    }
}
