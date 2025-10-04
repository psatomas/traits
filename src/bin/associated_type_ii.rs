use std::ops::Add;

fn add_two_numbers<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn main() {
    let integer_sum = add_two_numbers(1, 2);
    let float_sum = add_two_numbers(1.5, 2.4);
    println!("{integer_sum} and {float_sum}");
}