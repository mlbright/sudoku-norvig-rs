#[macro_use]
extern crate lazy_static;

use itertools::iproduct;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

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

    /*
    let grid1 = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";
    if let Some(solution) = solve(&grid1) {
        display(&solution);
    };

    let grid2 = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";
    if let Some(solution) = solve(&grid2) {
        display(&solution);
    };
    */

    /*
    let grid3 = "6..1.7..4..5.4.....27.6.....3...5.7..9..3...2...2...3......6..5....51..62..4..8..";
    if let Some(solution) = solve(&grid3) {
        display(&solution);
    };
    */

    // super hard or impossible?
    // let hard1 = ".....6....59.....82....8....45........3........6..3.54...325..6..................";

    // if let Some(solution) = solve(&hard1) {
    //    display(&solution);
    // };

    solve_all(&from_file("easy50.txt"), "easy");
    solve_all(&from_file("top95.txt"), "hard");
    solve_all(&from_file("hardest.txt"), "hardest");
}

fn display(solution: &HashMap<String, String>) {
    let mut show: String = "".to_string();
    for sq in SQUARES.iter() {
        show.push_str(solution.get(sq).expect("missing square?"));
    }
    println!("{}", show);
}

fn parse_grid(grid: &str) -> Option<HashMap<String, String>> {
    // To start, every square can be any digit; then assign values from the grid.
    let mut solution: HashMap<String, String> = HashMap::new();
    for s in SQUARES.iter() {
        solution.insert(s.clone(), DIGITS.to_string());
    }
    let puzzle = grid_values(grid);
    for (square, value) in &puzzle {
        if DIGITS.contains(value) {
            if !assign(&mut solution, square, value) {
                return None;
            }
        }
    }
    Some(solution)
}

fn grid_values(grid: &str) -> HashMap<String, String> {
    let mut valid = DIGITS.to_string();
    valid.push('.');
    valid.push('0');
    let mut puzzle: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    for c in grid.chars() {
        let value = c.to_string();
        if valid.contains(&value) {
            puzzle.entry(SQUARES[i].clone()).or_insert(value);
            i = i + 1;
        }
    }
    assert_eq!(puzzle.len(), 81);
    puzzle
}

fn assign(puzzle: &mut HashMap<String, String>, square: &String, value: &String) -> bool {
    let other_values: Vec<String> = puzzle
        .get(square)
        .unwrap()
        .chars()
        .filter(|x| x.to_string() != *value)
        .map(|x| x.to_string())
        .collect();
    for other_value in other_values.iter() {
        if !eliminate(puzzle, square, other_value) {
            return false;
        }
    }
    true
}

fn eliminate(puzzle: &mut HashMap<String, String>, square: &String, value: &String) -> bool {
    let values_at_square = puzzle.get(square).unwrap();

    if !values_at_square.contains(value) {
        return true; // Already eliminated
    }

    let reduced_possibilities = values_at_square.replace(value, "");
    puzzle.insert(square.clone(), reduced_possibilities.clone());

    if reduced_possibilities.len() == 0 {
        return false; // Contradiction, removed the last digit
    }

    // (1) If a square s is reduced to one value, then eliminate it from its peers.
    if reduced_possibilities.len() == 1 {
        for peer in PEERS.get(square).unwrap().iter() {
            if !eliminate(puzzle, peer, &reduced_possibilities) {
                return false;
            }
        }
    }

    // (2) If a unit u is reduced to only one place for a value d, then put it there.
    let units = UNITS.get(square).unwrap();
    for unit in units.iter() {
        let mut spots: Vec<String> = vec![];
        for sq in unit.iter() {
            if puzzle.get(sq).unwrap().contains(value) {
                spots.push(sq.clone());
            }
        }

        if spots.len() == 0 {
            return false; // Contradiction
        }

        if spots.len() == 1 {
            if !assign(puzzle, &spots[0], &value) {
                return false;
            }
        }
    }
    true
}

fn solve(grid: &str) -> Option<HashMap<String, String>> {
    match parse_grid(grid) {
        None => None,
        Some(puzzle) => search(Some(puzzle)),
    }
}

fn search(p: Option<HashMap<String, String>>) -> Option<HashMap<String, String>> {
    match p {
        None => None,
        Some(puzzle) => {
            let mut min_square = String::from("");
            let mut min_size = 10;

            for square in SQUARES.iter() {
                let size = puzzle.get(square).unwrap().len();
                if size > 1 && size < min_size {
                    min_square = square.clone();
                    min_size = size;
                }
            }

            if min_square == "" {
                return Some(puzzle);
            }

            for other_values in puzzle.get(&min_square).unwrap().chars() {
                let mut puzzle_copy = puzzle.clone();
                if assign(&mut puzzle_copy, &min_square, &other_values.to_string()) {
                    if let Some(result) = search(Some(puzzle_copy)) {
                        return Some(result);
                    }
                }
            }
            None
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

fn from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn solve_all(grids: &Vec<String>, name: &str) {
    let mut times: Vec<std::time::Duration> = vec![];

    for grid in grids.iter() {
        if let Some(t) = time_solve(grid) {
            times.push(t);
        }
    }

    let n = grids.len();
    let s = sum(&times);
    println!(
        "Solved {} of {} {} puzzles (avg. {:.4} secs ({:.2} Hz), max {:.4} secs).",
        times.len(),
        n,
        name,
        s.as_secs_f64() / n as f64,
        n as f64 / s.as_secs_f64(),
        max(&times).as_secs_f64(),
    );
}

fn time_solve(grid: &str) -> Option<std::time::Duration> {
    let start = Instant::now();
    if let Some(_) = solve(grid) {
        let duration = start.elapsed();
        return Some(duration);
    }
    None
}

fn max(durations: &Vec<std::time::Duration>) -> std::time::Duration {
    let mut max = Duration::new(0, 0);
    for duration in durations.iter() {
        if duration > &max {
            max = *duration;
        }
    }
    return max;
}

fn sum(durations: &Vec<std::time::Duration>) -> std::time::Duration {
    let mut sum = Duration::new(0, 0);
    for duration in durations.iter() {
        // println!("{}", duration.as_secs_f64());
        sum = sum + *duration;
    }
    return sum;
}

fn random_puzzle() {
    let mut puzzle: HashMap<String, String> = HashMap::new();
    for square in SQUARES.iter() {
        puzzle
            .entry(square.to_string())
            .or_insert(DIGITS.to_string());
    }
}
