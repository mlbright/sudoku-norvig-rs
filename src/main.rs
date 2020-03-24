use itertools::iproduct;

static DIGITS: &str = "123456789";
static ROWS: &str = "ABCDEFGHI";

fn main() {
    let squares: Vec<_> = cross(ROWS, DIGITS);
    assert_eq!(squares.len(), 81);
}

fn cross(a: &str, b: &str) -> Vec<String> {
    let mut cp = Vec::<String>::new();
    for (i, j) in iproduct!(a.chars(), b.chars()) {
        cp.push(format!("{}{}", i, j));
    }
    cp
}
