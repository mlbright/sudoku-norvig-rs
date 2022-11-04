use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::time::{Duration, Instant};
use sudoku::Sudoku;

fn main() {
    let mut random_puzzles: Vec<String> = vec![];
    let puzzle = Sudoku::new();
    for _ in 0..99 {
        random_puzzles.push(puzzle.random_puzzle());
    }
    // solve_all(&random_puzzles, "random");
    solve_all(&from_file("puzzles/easy50.txt"), "easy");
    solve_all(&from_file("puzzles/top95.txt"), "hard");
    solve_all(&from_file("puzzles/hardest.txt"), "hardest");
    solve_all(&from_file("puzzles/hardest20.txt"), "hardest20");
    solve_all(&from_file("puzzles/hardest20x50.txt"), "hardest20x50");
    solve_all(&from_file("puzzles/topn87.txt"), "topn87");
    solve_all(&from_file("puzzles/hardest-long.txt"), "most-difficult");
    solve_all(&from_file("puzzles/all.txt"), "combined");
}

fn from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn solve_all(grids: &[String], name: &str) {
    let n = grids.len();
    let mut times: Vec<Duration> = vec![];

    for grid in grids.iter() {
        if let Some(t) = time_solve(grid) {
            times.push(t);
        }
    }

    let mut sum = Duration::new(0, 0);
    let mut max = Duration::new(0, 0);
    for duration in times.iter() {
        sum += *duration;
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

fn time_solve(grid: &str) -> Option<Duration> {
    let solver = Sudoku::new();
    let start = Instant::now();
    if let Some(_solution) = solver.solve(grid) {
        let duration = start.elapsed();
        return Some(duration);
    }
    None
}
