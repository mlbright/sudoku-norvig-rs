use itertools::iproduct;

static DIGITS: &str = "12345679";
static ROWS: &str = "ABCDEFGHI";

fn main() {
    println!("Hello, world!");

    for (i, j) in iproduct!(ROWS.chars(), DIGITS.chars()) {
        println!("{}{}", i, j);
    }
}
