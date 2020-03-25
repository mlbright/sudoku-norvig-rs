use itertools::iproduct;

static DIGITS: &str = "123456789";
static ROWS: &str = "ABCDEFGHI";

fn main() {
    let squares: Vec<_> = cross(ROWS, DIGITS);
    assert_eq!(squares.len(), 81);

    let mut unitlist: std::vec::Vec<std::vec::Vec<std::string::String>> = vec![];

    for c in DIGITS.chars() {
        unitlist.push(cross(ROWS, &c.to_string()));
    }

    for r in ROWS.chars() {
        unitlist.push(cross(&r.to_string(), DIGITS));
    }

    let rs: Vec<_> = vec!["ABC".to_string(), "CDE".to_string(), "GHI".to_string()];
    let cs: Vec<_> = vec!["123".to_string(), "456".to_string(), "789".to_string()];
    for r in rs.iter() {
        for c in cs.iter() {
            unitlist.push(cross(r, c));
        }
    }

    println!("{:?}", unitlist);
    assert_eq!(unitlist.len(), 27);
}

fn cross(a: &str, b: &str) -> Vec<String> {
    let mut cp = Vec::<String>::new();
    for (i, j) in iproduct!(a.chars(), b.chars()) {
        cp.push(format!("{}{}", i, j));
    }
    cp
}
