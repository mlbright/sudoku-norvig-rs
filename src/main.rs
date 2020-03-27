#[macro_use]
extern crate lazy_static;

use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

static DIGITS: &str = "123456789";
static ROWS: &str = "ABCDEFGHI";

lazy_static! {
    static ref SQUARES: Vec<String> = cross(ROWS, DIGITS);
    static ref UNITLIST: std::vec::Vec<std::vec::Vec<std::string::String>> = {
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

        unitlist
    };
    static ref UNITS: HashMap<String, Vec<Vec<String>>> = {
        let mut units: HashMap<String, Vec<Vec<String>>> = HashMap::new();

        for s in SQUARES.iter() {
            let mut group: Vec<Vec<String>> = Vec::new();
            for unit in UNITLIST.iter() {
                for square in unit.iter() {
                    if square == s {
                        group.push(unit.clone());
                        break;
                    }
                }
            }
            units.insert(s.clone(), group);
        }

        units
    };
    static ref PEERS: HashMap<String, Vec<String>> = {
        let mut peers: HashMap<String, Vec<String>> = HashMap::new();

        for s in SQUARES.iter() {
            let mut peer_set: HashSet<String> = HashSet::new();
            for unit in UNITS.get(s).unwrap() {
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
        peers
    };
}

fn test() {
    assert_eq!(SQUARES.len(), 81);
    assert_eq!(UNITLIST.len(), 27);

    for s in SQUARES.iter() {
        match UNITS.get(s) {
            None => panic!("No units for {}", s),
            Some(ulist) => assert_eq!(ulist.len(), 3),
        }
    }

    for s in SQUARES.iter() {
        match PEERS.get(s) {
            None => panic!("No peers for {}", s),
            Some(plist) => assert_eq!(plist.len(), 20),
        }
    }

    // println!("{:?}", UNITS.get("C2").unwrap());

    // let mut test = PEERS.get("C2").unwrap().clone();
    // test.sort();
    // println!("{:?}", test);
}

fn main() {
    test();

    let grid1 = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
    let _grid2 =
        "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    let _hard1 =
        ".....6....59.....82....8....45........3........6..3.54...325..6..................";

    solve(grid1);
}

fn parse_grid(grid: &str) -> Option<HashMap<String, String>> {
    let mut solution: HashMap<String, String> = HashMap::new();
    for s in SQUARES.iter() {
        solution.insert(s.clone(), DIGITS.to_string());
    }
    Some(solution)
}

fn grid_values(grid: &str) -> HashMap<String, String> {
    let mut valid = DIGITS.to_string();
    valid.push_str(&'.'.to_string());
    println!("{}", valid);
    let mut puzzle: HashMap<String, String> = HashMap::new();
    let i = 0;
    for c in grid.chars() {
        let value = c.to_string();
        if valid.contains(&value) {
            puzzle.insert(SQUARES[i].clone(), value);
        }
    }
    assert_eq!(puzzle.len(),81);
    puzzle
}

fn solve(grid: &str) -> Option<HashMap<String, String>> {
    match parse_grid(grid) {
        None => None,
        Some(puzzle) => search(puzzle),
    }
}

fn search(puzzle: HashMap<String, String>) -> Option<HashMap<String, String>> {
    Some(puzzle)
}

fn cross(a: &str, b: &str) -> Vec<String> {
    let mut cp = Vec::<String>::new();
    for (i, j) in iproduct!(a.chars(), b.chars()) {
        cp.push(format!("{}{}", i, j));
    }
    cp
}

fn from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
