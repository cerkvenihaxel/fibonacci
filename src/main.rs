use std::io;

fn main() {
    println!("Please insert de first number greater than 0: ");
    
    let mut fib_range: String = String::new();

    io::stdin()
    .read_line(&mut fib_range)
    .expect("Failed to read line...");

    let fib_range: i32 = fib_range
    .trim()
    .parse()
    .expect("Please insert a number");

    for int in 0..=fib_range {
        println! ( "fibonacci ({}) => {}", int, fibonacci_number(int));
    }

}

fn fibonacci_number(x: i32) -> i32{
    if x < 0{
        return 0;
    }
    else if x == 1 {
        return 1;
    }
    else {
        return fibonacci_number(x-1) + fibonacci_number(x-2);
    }
}