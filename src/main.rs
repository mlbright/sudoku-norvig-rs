use itertools::iproduct;

static DIGITS: &str = "12345679";
static ROWS: &str = "ABCDEFGHI";

fn main() {
    println!("{:?}",cross(ROWS,DIGITS));
}

fn cross(a:&str,b:&str) -> Vec::<String> {
    let mut cp = Vec::<String>::new();
    for (i, j) in iproduct!(a.chars(), b.chars()) {
        cp.push(format!("{}{}", i, j));
    }
    cp
}
