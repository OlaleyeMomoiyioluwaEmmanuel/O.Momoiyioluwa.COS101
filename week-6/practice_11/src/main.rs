 fn main() {

Let a:1322;

// Bit presentation 10

Let b:1323;

// Bit presentation 11

Let mut result:132;

result a & b;

println!("(a & b) -> {} ", result);

result ab;

println!("(a | b) => {} ", result);

result a^ b;

println!("(ab) -> ()", result);

result Ib;

println!("(lb) -> () ", result);

result a <<< b; printin!("(a << b) -> ()", result);

result a >> b;

println!("(a >> b) -> ()", result);

}