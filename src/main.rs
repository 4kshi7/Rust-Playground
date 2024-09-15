struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height) 
    }
}

enum Shapes{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn calculate_area(shape: Shapes) -> f64 {
    match shape{
        Shapes::Circle(r) => 3.14159*r*r,
        Shapes::Square(s) => s*s,
        Shapes::Rectangle(w,h) => w*h,
    }
}


fn main() {
    println!("Hello, world!");
    println!("{}", is_even(3));
    println!("{}", fib(5));
    println!("{}", prime(6));
    println!("{}", factorial(5));
    println!("Length of string is : {}", string_length("abcdefghijklmnop"));


    //struct
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    println!("User 1 details \n\tSubscription : {} \n\tUsername : {} \n\tEmail : {} \n\tCount : {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    let rect = Rectangle{
        width: 25,
        height: 35,
    };

    println!("Area of rectangle is {} sq units", rect.area());
    println!("Perimeter of rectangle is {} units", rect.perimeter());

    //Enum
    let circle = Shapes::Circle(5.0);
    let square = Shapes::Square(4.0);
    let rectangle = Shapes::Rectangle(3.0, 6.0);
    println!("Area of Circle: {}", calculate_area(circle));
    println!("Area of Square: {}", calculate_area(square));
    println!("Area of Rectangle: {}", calculate_area(rectangle));
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

// a function that string as an input and return its length
fn string_length(s: &str) -> i32 {
    let mut count = 0; 
    for _ in s.chars() { 
        count += 1; 
    }

    //implicit return
    count
}