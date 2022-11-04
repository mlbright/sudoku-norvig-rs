mod cell;
use cell::Cell;

use array_macro::array;
use arrayvec::ArrayVec;
use rand::prelude::*;
use std::collections::HashMap;

static DIGITS: &str = "123456789";
static ROWS: &str = "ABCDEFGHI";

pub struct Sudoku {
    peers: Vec<Vec<usize>>,
    units: Vec<Vec<Vec<usize>>>,
    isquares: HashMap<String, usize>,
    squares: Vec<String>,
    #[allow(dead_code)]
    iunitlist: Vec<Vec<String>>,
    #[allow(dead_code)]
    unitlist: Vec<Vec<usize>>,
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sudoku {
    pub fn new() -> Self {
        let squares: Vec<String> = cross(ROWS, DIGITS);
        let mut isquares: HashMap<String, usize> = HashMap::new();
        for (i, sq) in squares.iter().enumerate() {
            isquares.insert(sq.clone(), i);
        }

        let mut iunitlist: Vec<Vec<String>> = vec![];

        for c in DIGITS.chars() {
            iunitlist.push(cross(ROWS, &c.to_string()));
        }

        for r in ROWS.chars() {
            iunitlist.push(cross(&r.to_string(), DIGITS));
        }

        let rs: ArrayVec<String, 3> =
            ArrayVec::from(["ABC".to_string(), "DEF".to_string(), "GHI".to_string()]);
        let cs: ArrayVec<String, 3> =
            ArrayVec::from(["123".to_string(), "456".to_string(), "789".to_string()]);

        for r in rs.iter() {
            for c in cs.iter() {
                iunitlist.push(cross(r, c));
            }
        }

        let mut unitlist: Vec<Vec<usize>> = vec![];
        for unit in iunitlist.iter() {
            let mut u: Vec<usize> = vec![];
            for sq in unit.iter() {
                let d = isquares.get(sq).unwrap();
                u.push(*d);
            }
            unitlist.push(u);
        }

        let mut units: Vec<Vec<Vec<usize>>> = Vec::with_capacity(81);
        for (i, _) in squares.iter().enumerate() {
            let mut group: Vec<Vec<usize>> = vec![];
            for unit in unitlist.iter() {
                for j in unit.iter() {
                    if i == *j {
                        group.push(unit.clone());
                        break;
                    }
                }
            }
            units.push(group);
        }

        let mut peers: Vec<Vec<usize>> = Vec::with_capacity(20);
        for (i, _) in squares.iter().enumerate() {
            let mut peer_set: Vec<usize> = vec![];
            for unit in units[i].iter() {
                for square in unit.iter() {
                    if *square != i {
                        peer_set.push(*square);
                    }
                }
            }
            peer_set.sort_unstable();
            peer_set.dedup();
            peers.push(peer_set);
        }

        Sudoku {
            peers,
            units,
            isquares,
            squares,
            iunitlist,
            unitlist,
        }
    }

    fn blank_puzzle(&self) -> [Cell; 81] {
        array![Cell::new(); 81]
    }

    pub fn format_grid(&self, solution: &[Cell]) -> String {
        let mut show = String::new();
        for item in solution.iter().take(81) {
            if item.len() == 1 {
                let v = &item.first();
                show.push_str(&v.to_string());
            } else {
                show.push('.');
            }
        }
        show
    }

    pub fn parse_grid(&self, grid: &str) -> Option<[Cell; 81]> {
        // To start, every square can be any digit; then assign values from the grid.
        let mut solution = self.blank_puzzle();
        for (i, c) in grid.chars().enumerate() {
            if c.is_ascii_digit() && c != '0' {
                let d = c.to_digit(10).unwrap() as usize;
                if !self.assign(&mut solution, i, d) {
                    return None;
                }
            }
        }
        Some(solution)
    }

    fn assign(&self, puzzle: &mut [Cell], square: usize, value_to_assign: usize) -> bool {
        for d in 1..=9 {
            if d != value_to_assign
                && puzzle[square].contains(d)
                && !self.eliminate(puzzle, square, d)
            {
                return false;
            }
        }
        true
    }

    fn eliminate(&self, puzzle: &mut [Cell], square: usize, value_to_eliminate: usize) -> bool {
        if !puzzle[square].contains(value_to_eliminate) {
            return true;
        }

        puzzle[square].remove(value_to_eliminate);

        if puzzle[square].len() == 0 {
            return false; // Contradiction, removed the last digit
        }

        // (1) If a square s is reduced to one value, then eliminate it from its peers.
        if puzzle[square].len() == 1 {
            let last_value = puzzle[square].first();
            for peer in self.peers[square].iter() {
                if !self.eliminate(puzzle, *peer, last_value) {
                    return false;
                }
            }
        }

        // (2) If a unit u is reduced to only one place for a value d, then put it there.
        for unit in self.units[square].iter() {
            let spots = unit
                .iter()
                .filter(|sq| puzzle[**sq].contains(value_to_eliminate))
                .copied()
                .collect::<ArrayVec<usize, 9>>();

            if spots.is_empty() {
                return false; // Contradiction
            }

            if spots.len() == 1 && !self.assign(puzzle, spots[0], value_to_eliminate) {
                return false;
            }
        }
        true
    }

    pub fn solve(&self, grid: &str) -> Option<[Cell; 81]> {
        match self.parse_grid(grid) {
            None => None,
            Some(puzzle) => self.search(Some(puzzle)),
        }
    }

    fn search(&self, p: Option<[Cell; 81]>) -> Option<[Cell; 81]> {
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
                        if min_size == 2 {
                            break;
                        }
                    }
                }

                if min_square == 82 {
                    return Some(puzzle);
                }

                for value in 1..=9 {
                    if puzzle[min_square].contains(value) {
                        let mut puzzle_copy = puzzle.clone();
                        if self.assign(&mut puzzle_copy, min_square, value) {
                            if let Some(result) = self.search(Some(puzzle_copy)) {
                                return Some(result);
                            }
                        }
                    }
                }
                None
            }
        }
    }

    pub fn random_puzzle(&self) -> String {
        let mut rng = thread_rng();
        let mut puzzle: [Cell; 81] = self.blank_puzzle();
        let mut random_squares = self.squares.clone();
        random_squares.shuffle(&mut rng);

        for random_square in random_squares.iter() {
            let random_index = self.isquares.get(random_square).unwrap();
            let possible_values = puzzle[*random_index].clone();

            for d in 1..=9 {
                if possible_values.contains(d) && !self.assign(&mut puzzle, *random_index, d) {
                    break;
                }
            }

            let mut successfully_assigned = vec![];
            for item in puzzle.iter().take(self.squares.len()) {
                if item.len() == 1 {
                    successfully_assigned.push(item.first());
                }
            }

            let mut digits = successfully_assigned.clone();
            digits.dedup();

            if successfully_assigned.len() >= 17 && digits.len() >= 8 {
                return self.format_grid(&puzzle);
            }
        }

        self.random_puzzle()
    }
}

fn cross(a: &str, b: &str) -> Vec<String> {
    let mut cp = Vec::<String>::new();
    use itertools::iproduct;
    for (i, j) in iproduct!(a.chars(), b.chars()) {
        cp.push(format!("{}{}", i, j));
    }
    cp
}

#[cfg(test)]
mod tests {
    #[test]
    fn assertions() {
        let puzzle = crate::Sudoku::new();
        assert_eq!(puzzle.squares.len(), 81);
        assert_eq!(puzzle.isquares.len(), 81);
        assert_eq!(puzzle.iunitlist.len(), 27);
        assert_eq!(puzzle.unitlist.len(), 27);
        assert_eq!(puzzle.unitlist[0].len(), 9);

        for (i, _) in puzzle.squares.iter().enumerate() {
            assert_eq!(puzzle.units[i].len(), 3);
        }

        for (i, _) in puzzle.squares.iter().enumerate() {
            assert_eq!(puzzle.peers[i].len(), 20);
        }
    }
}
