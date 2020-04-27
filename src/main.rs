#[macro_use]
extern crate lazy_static;
extern crate rand;
#[macro_use]
extern crate smallbitvec;

mod cell;
use cell::Cell;

use itertools::iproduct;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
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
    static ref ISQUARES: HashMap<String, usize> = {
        let mut isquares: HashMap<String, usize> = HashMap::new();
        for (i, sq) in SQUARES.iter().enumerate() {
            isquares.insert(sq.clone(), i);
        }
        isquares
    };
    static ref BLANK_PUZZLE: Vec<Cell> = { vec![Cell::new(); 81] };
    static ref IUNITLIST: Vec<Vec<String>> = {
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
    static ref UNITLIST: std::vec::Vec<Vec<usize>> = {
        let mut unitlist: Vec<Vec<usize>> = vec![];
        for unit in IUNITLIST.iter() {
            let mut u: Vec<usize> = vec![];
            for sq in unit.iter() {
                let d = ISQUARES.get(sq).unwrap();
                u.push(*d);
            }
            unitlist.push(u);
        }
        unitlist
    };
    static ref UNITS: Vec<Vec<Vec<usize>>> = {
        let mut units: Vec<Vec<Vec<usize>>> = Vec::with_capacity(81);
        for (i, _) in SQUARES.iter().enumerate() {
            let mut group: Vec<Vec<usize>> = vec![];
            for unit in UNITLIST.iter() {
                for j in unit.iter() {
                    if i == *j {
                        group.push(unit.clone());
                        break;
                    }
                }
            }
            units.push(group);
        }

        units
    };
    static ref PEERS: Vec<Vec<usize>> = {
        let mut peers: Vec<Vec<usize>> = Vec::with_capacity(20);
        for (i, _) in SQUARES.iter().enumerate() {
            let mut peer_set: Vec<usize> = vec![];
            for unit in UNITS[i].iter() {
                for square in unit.iter() {
                    if *square != i {
                        peer_set.push(*square);
                    }
                }
            }
            peer_set.sort();
            peer_set.dedup();
            peers.push(peer_set);
        }
        peers
    };
}

fn test() {
    assert_eq!(SQUARES.len(), 81);
    assert_eq!(ISQUARES.len(), 81);
    assert_eq!(IUNITLIST.len(), 27);
    assert_eq!(UNITLIST.len(), 27);

    for (i, _) in SQUARES.iter().enumerate() {
        assert_eq!(UNITS[i].len(), 3);
    }

    for (i, _) in SQUARES.iter().enumerate() {
        assert_eq!(PEERS[i].len(), 20);
    }
}

fn main() {
    test();
    solve_all(&from_file("easy50.txt"), "easy");
    solve_all(&from_file("top95.txt"), "hard");
    solve_all(&from_file("hardest.txt"), "hardest");
    let mut random_puzzles: Vec<String> = vec![];
    for _ in 0..99 {
        random_puzzles.push(random_puzzle());
    }
    solve_all(&random_puzzles, "random");
}

fn format_grid(solution: &Vec<Cell>) -> String {
    let mut show: String = "".to_string();
    for i in 0..SQUARES.len() {
        if solution[i].len() == 1 {
            show.push_str(&solution[i].to_string());
        } else {
            show.push('.');
        }
    }
    show
}

fn parse_grid(grid: &str) -> Option<Vec<Cell>> {
    // To start, every square can be any digit; then assign values from the grid.
    let mut solution = BLANK_PUZZLE.clone();
    for (i, c) in grid.chars().enumerate() {
        if c.is_ascii_digit() && c != '0' {
            let d = c.to_digit(10).unwrap() as usize;
            if !assign(&mut solution, i, d - 1) {
                return None;
            }
        }
    }
    Some(solution)
}

fn assign(puzzle: &mut [Cell], square: usize, value_to_assign: usize) -> bool {
    puzzle[square]
        .possibilities()
        .iter()
        .filter(|d| **d != value_to_assign)
        .map(|d| *d)
        .collect::<Vec<usize>>()
        .iter()
        .all(|c| eliminate(puzzle, square, *c))
}

fn eliminate(puzzle: &mut [Cell], square: usize, value_to_eliminate: usize) -> bool {
    if !puzzle[square].contains(value_to_eliminate) {
        return true;
    }

    puzzle[square].remove(value_to_eliminate);

    if puzzle[square].len() == 0 {
        return false; // Contradiction, removed the last digit
    }

    // (1) If a square s is reduced to one value, then eliminate it from its peers.
    if puzzle[square].len() == 1 {
        for peer in PEERS[square].iter() {
            if !eliminate(
                puzzle,
                *peer,
                *puzzle[square].possibilities().iter().nth(0).unwrap(),
            ) {
                return false;
            }
        }
    }

    // (2) If a unit u is reduced to only one place for a value d, then put it there.
    for unit in UNITS[square].iter() {
        let mut spots: Vec<usize> = vec![];
        for sq in unit {
            if puzzle[*sq].contains(value_to_eliminate) {
                spots.push(*sq);
            }
        }

        if spots.len() == 0 {
            return false; // Contradiction
        }

        if spots.len() == 1 {
            if !assign(puzzle, spots[0], value_to_eliminate) {
                return false;
            }
        }
    }
    true
}

fn solve(grid: &str) -> Option<Vec<Cell>> {
    match parse_grid(grid) {
        None => None,
        Some(puzzle) => search(Some(puzzle)),
    }
}

fn search(p: Option<Vec<Cell>>) -> Option<Vec<Cell>> {
    match p {
        None => None,
        Some(puzzle) => {
            let mut min_square = 82;
            let mut min_size = 10;

            for (i, square) in puzzle.iter().enumerate() {
                let size = square.len();
                if size > 1 && size < min_size {
                    min_square = i;
                    min_size = size;
                }
            }

            if min_square == 82 {
                return Some(puzzle);
            }

            for other_value in puzzle[min_square].possibilities() {
                let mut puzzle_copy = puzzle.clone();
                if assign(&mut puzzle_copy, min_square, other_value) {
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
    let n = grids.len();
    let mut times: Vec<std::time::Duration> = vec![];

    for grid in grids.iter() {
        if let Some(t) = time_solve(grid) {
            times.push(t);
        }
    }

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
    let mut puzzle: Vec<Cell> = BLANK_PUZZLE.clone();
    let mut random_squares = SQUARES.clone();
    random_squares.shuffle(&mut rng);

    for random_square in random_squares.iter() {
        let random_index = ISQUARES.get(random_square).unwrap();
        let square_possibilities = puzzle[*random_index].clone();
        let i = rng.gen_range(0, square_possibilities.len());

        if !assign(
            &mut puzzle,
            *random_index,
            *square_possibilities.possibilities().iter().nth(i).unwrap(),
        ) {
            break;
        }

        let mut successfully_assigned = vec![];
        for square in 0..SQUARES.len() {
            if puzzle[square].len() == 1 {
                successfully_assigned.push(*puzzle[square].possibilities().iter().nth(0).unwrap());
            }
        }

        let mut digits = successfully_assigned.clone();
        digits.dedup();

        if successfully_assigned.len() >= 17 && digits.len() >= 8 {
            return format_grid(&puzzle);
        }
    }

    return random_puzzle();
}
