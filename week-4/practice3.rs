Another Standard Input

// Rust program to calculate the area of a triangle given three side

use std::io;

Practice 2: week-4/pr

fn main()

{

Let mut input1- String::new();

Let mut input2 String::new(); Let mut input3 String::new();

printin!("Enter first edge of triangle: "); 10::stdin().read_Line(&mut input1).expect("Not a valid string"); Let a:f32 input1.trim().parse().expect("Not a valid number");

printin!("Enter second edge of triangle: "); io::stdin().read_Line(&mut input2).expect("Not a valid string"); Let b:f32 input2.trim().parse().expect("Not a valid number");

printin! ("Enter third edge of triangle: "); io::stdin().read_Line(&mut input3).expect("Not a valid string"); let c:f32 input3.trim().parse().expect("Not a valid number");

let s:f32 (a+b+c) Let mut area:f32s (sa) (sb) (sc); area area.sqrt();

printin!("Area of a triangle: ()", area);
}

