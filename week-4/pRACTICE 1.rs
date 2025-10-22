// Rust program to output name and age

use std::io;

fn main() {

println("\nStudent Information Management System!");

// input name

printin!("\nPlease Enter your name.");

Let mut name String::new();

10::stdin()

.read_Line(&mut name)

.expect("Failed to read input"); printin!("Your name is: {}", name);

// input age

println("\nEnter your age.");

Let mut age String::new();

io::stdin().read_Line(&mut age).expect("Failed to read input"); Let age: 132 age.trim().parse().expect("Input not an integer"); printin!("Your age is: ()", age);

}