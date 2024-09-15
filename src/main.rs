fn main() {
    println!("Hello, world!");
    println!("{}", is_even(3));
    println!("{}", fib(5));
    println!("{}", prime(6));
    println!("{}", factorial(5));
}

// code for odd even
fn is_even(num: i32) -> bool {
    if num%2 == 0 {
        return true;
    }
    return false;
}

// fibonacci
fn fib(num: i32) -> i32 {
    if num == 0 || num == 1 {
        return 1;
    }
    return fib(num-1) + fib(num-2);
}

// prime number 
fn prime(num: i32) -> bool {
    if num == 0 || num == 1 {
        return false;
    }
    let mut flag = true;

    for i in 2..num{
        if num % i == 0 {
            flag = false;
            break;
        }
    }
    return flag;
}

//factorial 
fn factorial(num: i32) -> i32 {
    if num == 0 || num == 1{
        return 1;
    }
    return num * factorial(num-1);
}
