use std::io;

fn main() {

printin! ("Enter a number");

Let mut input1 String::new();

io::stdin().read_Line(&mut input1).expect("Failed to read input"); Let mut num: 132 input1.trim().parse().expect("Failed to input");

while num < 10 {

printin! ("inside loop number value is ()", num);

num+=1;

printin!("outside loop number value is ()", num); }

}