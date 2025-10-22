// Rust program to determine age pass

use std::io;

fn main() {

}

Let mut input1 String::new();

Let mut input2 String::new();

printin!("Enter your name: ");

io::stdin().read_Line(&mut input1).expect(llot a valid string");

printin!("Enter your age: ");

10::stdin().read_Line(&mut input2).expect("liot a valid string");

Let age: 132 input2.trim().parse().expect("liot a valid number");

if age >= 18 {

printin!("Welcome to the party {}!", input1);

} else {

} println!("Oops, you are not of age to enter the party {}", input1);