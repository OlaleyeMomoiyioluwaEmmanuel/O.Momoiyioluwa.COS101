

// Rust program to calculate the area of a

// triangle for a given base and height

use std::io;

fn main() {

Let mut inputi - String::new(); Let mut input2 - String::new();

printin!("Enter base: "); io::stdin().read_Line(&mut input1).expect("Not a valid string"); Let base:f32 input1.trim().parse().expect("Not a valid number");

printin!("Enter hel height: "); io::stdin().read_Line(&mut input2).expect("Not a valid string"); Let height:f32-input2.trim().parse().expect("Hot a valid number");

if base > 0.0 { printin!("Area of a triangle: ()", area); Let area:f32 -(base height) / 2.0; }

}