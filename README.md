# Rust Playground

A simple Rust project for experimenting and learning Rust programming. 

## Getting Started

1. **Install Rust**: Follow instructions on [the official Rust website](https://www.rust-lang.org/).
2. **Clone the Repository**:
    ```sh
    git clone https://github.com/yourusername/rust-playground.git
    cd rust-playground
    ```
3. **Build and Run**:
    ```sh
    cargo build
    cargo run
    ```

# Memory Management, Ownership, Borrowing, and References in Rust

Rust provides a unique approach to memory management with a set of rules that ensure memory safety without needing a garbage collector. In this guide, we'll cover the core concepts of Ownership, Borrowing, and References in Rust.

## 1. Ownership
Ownership is Rust's way of managing memory. Each value in Rust has a *single owner*, and when the owner goes out of scope, the value is dropped and memory is freed.

### Example:
```rust
fn main() {
    let s1 = String::from("Hello, Rust!"); // s1 owns the String
    let s2 = s1; // Ownership is moved to s2

    // println!("{}", s1); // This will cause an error because s1 no longer owns the value
    println!("{}", s2); // Valid, s2 is the current owner
}
```
In the example above, `s1` initially owns the string, but after the line `let s2 = s1;`, ownership is moved to `s2`. Trying to use `s1` will result in a compile-time error.

## 2. Borrowing and References
Borrowing allows us to access a value without taking ownership. We do this using references (`&`). This enables us to read or modify data without transferring ownership.

### Example:
```rust
fn main() {
    let s1 = String::from("Hello, Rust!");
    
    // Immutable reference
    let len = calculate_length(&s1); // Borrowing s1 using &
    println!("The length of '{}' is {}.", s1, len); // s1 can still be used

    // Mutable reference
    let mut s2 = String::from("Hello");
    add_world(&mut s2); // Borrowing s2 mutably using &mut
    println!("{}", s2); // s2 is changed to "Hello, World!"
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len() // Use the reference without taking ownership
}

fn add_world(s: &mut String) { // s is a mutable reference
    s.push_str(", World!"); // Modify the value s refers to
}
```
### Key Points:
- You can have *multiple immutable references* to a value, but only *one mutable reference* at a time.
- Mutable and immutable references cannot coexist for the same value.

## 3. Rules of Ownership and Borrowing
1. Each value in Rust has a *single owner*.
2. When the owner goes out of scope, the value is *dropped* and memory is freed.
3. You can create *immutable references* (`&T`) that allow read-only access.
4. You can create a *mutable reference* (`&mut T`) that allows modifying the value.
5. You cannot have a mutable reference while immutable references exist.

## 4. Understanding Scopes
Let's demonstrate how scopes impact borrowing and references:

### Example:
```rust
fn main() {
    let mut s = String::from("hello");

    { 
        let r1 = &s; // Immutable reference
        let r2 = &s; // Another immutable reference
        println!("{}, {}", r1, r2); // This is valid
    } // r1 and r2 go out of scope here

    let r3 = &mut s; // Mutable reference allowed since r1 and r2 are out of scope
    r3.push_str(", world");
    println!("{}", r3); // Prints "hello, world"
}
```
In this example, we have immutable references (`r1` and `r2`) first, but they are out of scope before we create the mutable reference `r3`. Rust's strict rules prevent data races and ensure safe memory access.

## 5. Summary
- **Ownership** is Rust’s way of ensuring memory safety.
- **Borrowing** lets us reference a value without taking ownership.
- **References** can be immutable (`&`) or mutable (`&mut`).
- **Memory safety** is guaranteed by compile-time checks.

These principles make Rust a powerful language for safe, concurrent programming. If you follow the ownership and borrowing rules, you can write fast and safe programs without worrying about memory leaks or data races.
---


## Contributing

Feel free to submit pull requests with improvements or new examples.
