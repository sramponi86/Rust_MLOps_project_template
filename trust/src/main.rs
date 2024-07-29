use trust::add;
use trust::sub;
use trust::mul;
use trust::div;

fn main() {
    println!("1 + 2 = {}", add(1, 2));
    println!("2 - 1 = {}", sub(2, 1));
    println!("1 * 2 = {}", mul(1, 2));
    println!("2 / 1 = {}", div(2, 1));
}