use lib_a::add_one;
use lib_b::add_two;

fn main() {
    let num = 5;
    let result_a = add_one(num);
    let result_b = add_two(num);

    println!("{} + 1 = {}", num, result_a);
    println!("{} + 2 = {}", num, result_b);
}