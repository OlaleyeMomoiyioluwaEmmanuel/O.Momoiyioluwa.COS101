use std::io;

fn main() {
    println!("Enter the coefficients of the quadratic equation (ax^2 + bx + c = 0):");

    println!("Enter a:");
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("Failed to read line");
    let a: f64 = a_str.trim().parse().expect("Invalid input");

    println!("Enter b:");
    let mut b_str = String::new();
    io::stdin().read_line(&mut b_str).expect("Failed to read line");
    let b: f64 = b_str.trim().parse().expect("Invalid input");

    println!("Enter c:");
    let mut c_str = String::new();
    io::stdin().read_line(&mut c_str).expect("Failed to read line");
    let c: f64 = c_str.trim().parse().expect("Invalid input");

    find_roots(a, b, c);
}