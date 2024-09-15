fn main() {
    println!("Hello, world!");
    println!("{}", is_even(3));
    println!("{}", fib(5));
}

fn is_even(num: i32) -> bool {
    if num%2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: i32) -> i32 {
    if num == 0 || num == 1 {
        return 1;
    }
    return fib(num-1) + fib(num-2);
}

