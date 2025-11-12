use std :: io;

fn add(a: i32, b: i32) {
Let sum = a + b;

println! ("Sum of A and B = {}", sum);

fn main() {

    Let mut input1 = String :: new();
    println!("Enter input for parameter A:");
    io :: stdin().read_line(&mut input1).expect("Failed to read input");
    Let a:i32 = input1.trim().parse().expect("Invalid input");

    Let mut input2 = String :: new();
    println! ("Enter input for parameter B:");
    io :: stdin().read_line(&mut input2).expect("Failed to read input");
    Let b:i32 = input2.trim().parse().expect("Invalid input");

// call add function with arguments
add(a,b);
}