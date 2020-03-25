use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;

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

    let rs: Vec<_> = vec!["ABC".to_string(), "DEF".to_string(), "GHI".to_string()];
    let cs: Vec<_> = vec!["123".to_string(), "456".to_string(), "789".to_string()];
    for r in rs.iter() {
        for c in cs.iter() {
            unitlist.push(cross(r, c));
        }
    }

    assert_eq!(unitlist.len(), 27);

    let mut units: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for s in squares.iter() {
        let mut group: Vec<Vec<String>> = Vec::new();
        for unit in unitlist.iter() {
            for square in unit.iter() {
                if square == s {
                    group.push(unit.clone());
                    break;
                }
            }
        }
        units.insert(s.clone(), group);
    }

    // println!("{:?}", units.get("C2").unwrap());
    for s in squares.iter() {
        match units.get(s) {
            None => panic!("No units for {}", s),
            Some(ulist) => assert_eq!(ulist.len(), 3),
        }
    }

    let mut peers: HashMap<String, Vec<String>> = HashMap::new();

    for s in squares.iter() {
        let mut peer_set: HashSet<String> = HashSet::new();
        for unit in units.get(s).unwrap() {
            for square in unit.iter() {
                if square != s {
                    peer_set.insert(square.clone());
                }
            }
        }
        let mut peer_list: Vec<String> = Vec::new();
        for peer in &peer_set {
            peer_list.push(peer.clone());
        }
        peers.insert(s.clone(), peer_list);
    }

    // let mut test = peers.get("C2").unwrap().clone();
    // test.sort();
    // println!("{:?}", test);
    for s in squares.iter() {
        match peers.get(s) {
            None => panic!("No peers for {}", s),
            Some(plist) => assert_eq!(plist.len(), 20),
        }
    }
}

fn cross(a: &str, b: &str) -> Vec<String> {
    let mut cp = Vec::<String>::new();
    for (i, j) in iproduct!(a.chars(), b.chars()) {
        cp.push(format!("{}{}", i, j));
    }
    cp
}
