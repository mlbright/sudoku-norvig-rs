#[macro_use]
extern crate lazy_static;
extern crate rand;

use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::Duration;
use std::time::Instant;

static DIGITS: &str = "123456789";
static VALID: &str = ".0123456789";
static ROWS: &str = "ABCDEFGHI";
static PUZZLE_N: usize = 17;

lazy_static! {
    static ref SQUARES: Vec<String> = cross(ROWS, DIGITS);
    static ref UNITLIST: Vec<Vec<String>> = {
        let mut unitlist: Vec<Vec<String>> = vec![];

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

    // TODO: need assertions for units["C2"] and peers["C2"]
    // println!("{:?}", UNITS.get("C2").unwrap());
    // let mut test = PEERS.get("C2").unwrap().clone();
    // test.sort();
    // println!("{:?}", test);
}

fn main() {
    test();
    solve_all(&from_file("easy50.txt"), "easy");
    solve_all(&from_file("top95.txt"), "hard");
    solve_all(&from_file("hardest.txt"), "hardest");

    let mut random_puzzles: Vec<String> = vec![];
    for _ in 0..100 {
        random_puzzles.push(random_puzzle());
    }

    solve_all(&random_puzzles, "random");
}

fn format_grid(solution: &HashMap<String, String>) -> String {
    let mut show: String = "".to_string();
    for sq in SQUARES.iter() {
        let v = solution.get(sq).unwrap();
        if v.len() == 1 {
            show.push_str(v);
        } else {
            show.push('.');
        }
    }
    show
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
    let mut puzzle: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    for c in grid.chars() {
        let value = c.to_string();
        if VALID.contains(&value) {
            puzzle.entry(SQUARES[i].clone()).or_insert(value);
            i = i + 1;
        }
    }
    assert_eq!(puzzle.len(), 81);
    puzzle
}

fn assign(puzzle: &mut HashMap<String, String>, square: &String, value: &String) -> bool {
    let other_values = puzzle.get(square).unwrap().clone();
    for other_value in other_values.chars() {
        let s = other_value.to_string();
        if s != *value {
            if !eliminate(puzzle, square, &s) {
                return false;
            }
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

    if reduced_possibilities.len() == 0 {
        return false; // Contradiction, removed the last digit
    }

    puzzle.insert(square.clone(), reduced_possibilities.clone());

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

    let mut sum = Duration::new(0, 0);
    let mut max = Duration::new(0, 0);
    for duration in times.iter() {
        sum = sum + *duration;
        if duration > &max {
            max = *duration;
        }
    }

    println!(
        "Solved {} of {} {} puzzles (avg. {:.4} secs ({:.2} Hz), max {:.4} secs).",
        times.len(),
        n,
        name,
        sum.as_secs_f64() / n as f64,
        n as f64 / sum.as_secs_f64(),
        max.as_secs_f64(),
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

fn random_puzzle() -> String {
    let mut rng = rand::thread_rng();
    let mut puzzle: HashMap<String, String> = HashMap::new();
    for square in SQUARES.iter() {
        puzzle
            .entry(square.to_string())
            .or_insert(DIGITS.to_string());
    }

    let mut random_squares = SQUARES.clone();
    random_squares.shuffle(&mut rng);

    for random_square in random_squares.iter() {
        let square_possibilities = puzzle.get(random_square).unwrap().clone();
        let i = rng.gen_range(0, square_possibilities.len());

        if !assign(
            &mut puzzle,
            &random_square,
            &square_possibilities.chars().nth(i).unwrap().to_string(),
        ) {
            break;
        }

        let mut successfully_assigned: String = String::from("");
        for square in SQUARES.iter() {
            let values_at_square = puzzle.get(square).unwrap();
            if values_at_square.len() == 1 {
                successfully_assigned.push_str(values_at_square);
            }
        }

        let mut chars: Vec<char> = successfully_assigned.chars().collect();
        chars.dedup();
        let deduped: String = chars.iter().collect();

        if successfully_assigned.len() >= PUZZLE_N && deduped.len() >= 8 {
            return format_grid(&puzzle);
        }
    }

    return random_puzzle();
}
