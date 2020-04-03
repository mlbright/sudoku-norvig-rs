# sudoku-norvig-rs

This repository is a Rust version of [Peter Norvig's "Solve Every Sudoku Puzzle"][original].

## Run

Install Rust with [rustup][rustup], which installs `cargo`.
Then run:

```
cargo run --release
```

## Performance

### MacOS

On my MacBook Pro, I get:

```
➜  sudoku-norvig-rs git:(master) python sudoku.py
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0034 secs (291.37 Hz), max 0.0047 secs).
Solved 95 of 95 hard puzzles (avg 0.0116 secs (85.94 Hz), max 0.0574 secs).
Solved 11 of 11 hardest puzzles (avg 0.0047 secs (211.70 Hz), max 0.0070 secs).
Solved 98 of 99 random puzzles (avg 0.0035 secs (285.60 Hz), max 0.0068 secs).
➜  sudoku-norvig-rs git:(master) ./sugoku # go build sugoku.go
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0007 secs (1378.21 Hz), max 0.0012 secs).
Solved 95 of 95 hard puzzles (avg 0.0020 secs (506.36 Hz), max 0.0091 secs).
Solved 11 of 11 hardest puzzles (avg 0.0008 secs (1259.45 Hz), max 0.0012 secs).
Solved 99 of 99 random puzzles (avg 0.0006 secs (1616.38 Hz), max 0.0011 secs).
➜  sudoku-norvig-rs git:(master) ./target/release/sudoku-norvig-rs
Solved 50 of 50 easy puzzles (avg. 0.0010 secs (1048.80 Hz), max 0.0012 secs).
Solved 95 of 95 hard puzzles (avg. 0.0030 secs (329.85 Hz), max 0.0161 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0012 secs (869.51 Hz), max 0.0015 secs).
Solved 99 of 99 random puzzles (avg. 0.0011 secs (873.16 Hz), max 0.0022 secs).
➜  sudoku-norvig-rs git:(master)
```

### Linux

On a Ubuntu 16.04.6 LTS machine, I get:

```
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ python sudoku.py
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0058 secs (173.57 Hz), max 0.0134 secs).
Solved 95 of 95 hard puzzles (avg 0.0223 secs (44.84 Hz), max 0.0990 secs).
Solved 11 of 11 hardest puzzles (avg 0.0066 secs (151.90 Hz), max 0.0091 secs).
Solved 99 of 99 random puzzles (avg 0.0063 secs (157.92 Hz), max 0.0124 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./sugoku
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0010 secs (999.13 Hz), max 0.0022 secs).
Solved 95 of 95 hard puzzles (avg 0.0030 secs (336.32 Hz), max 0.0146 secs).
Solved 11 of 11 hardest puzzles (avg 0.0012 secs (828.35 Hz), max 0.0016 secs).
Solved 99 of 99 random puzzles (avg 0.0010 secs (1043.60 Hz), max 0.0014 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./target/release/sudoku-norvig-rs
Solved 50 of 50 easy puzzles (avg. 0.0008 secs (1244.66 Hz), max 0.0012 secs).
Solved 95 of 95 hard puzzles (avg. 0.0025 secs (397.21 Hz), max 0.0119 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0010 secs (1008.18 Hz), max 0.0016 secs).
Solved 99 of 99 random puzzles (avg. 0.0009 secs (1089.24 Hz), max 0.0020 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$
```


[original]: http://norvig.com/sudoku.html
[rustup]: https://www.rust-lang.org/tools/install
