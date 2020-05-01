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
Solved 50 of 50 easy puzzles (avg 0.0036 secs (281.22 Hz), max 0.0054 secs).
Solved 95 of 95 hard puzzles (avg 0.0119 secs (84.38 Hz), max 0.0567 secs).
Solved 11 of 11 hardest puzzles (avg 0.0048 secs (208.86 Hz), max 0.0066 secs).
Solved 99 of 99 random puzzles (avg 0.0035 secs (285.29 Hz), max 0.0045 secs).
➜  sudoku-norvig-rs git:(master) ./sugoku
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0007 secs (1428.20 Hz), max 0.0015 secs).
Solved 95 of 95 hard puzzles (avg 0.0020 secs (498.01 Hz), max 0.0094 secs).
Solved 11 of 11 hardest puzzles (avg 0.0008 secs (1277.44 Hz), max 0.0014 secs).
Solved 99 of 99 random puzzles (avg 0.0006 secs (1599.51 Hz), max 0.0012 secs).
➜  sudoku-norvig-rs git:(master) ./target/release/sudoku-norvig-rs
All tests pass.
Solved 50 of 50 easy puzzles (avg. 0.0003 secs (3696.57 Hz), max 0.0004 secs).
Solved 95 of 95 hard puzzles (avg. 0.0008 secs (1231.66 Hz), max 0.0041 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0003 secs (3151.02 Hz), max 0.0006 secs).
Solved 99 of 99 random puzzles (avg. 0.0003 secs (3717.31 Hz), max 0.0004 secs).
➜  sudoku-norvig-rs git:(master)
```

### Linux

On a Ubuntu 16.04.6 LTS machine, I get:

```
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ python sudoku.py
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0049 secs (202.62 Hz), max 0.0060 secs).
Solved 95 of 95 hard puzzles (avg 0.0176 secs (56.82 Hz), max 0.0892 secs).
Solved 11 of 11 hardest puzzles (avg 0.0062 secs (160.76 Hz), max 0.0090 secs).
Solved 98 of 99 random puzzles (avg 0.0051 secs (194.32 Hz), max 0.0061 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./sugoku
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0009 secs (1072.89 Hz), max 0.0018 secs).
Solved 95 of 95 hard puzzles (avg 0.0028 secs (355.93 Hz), max 0.0130 secs).
Solved 11 of 11 hardest puzzles (avg 0.0011 secs (934.15 Hz), max 0.0014 secs).
Solved 99 of 99 random puzzles (avg 0.0009 secs (1109.33 Hz), max 0.0033 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./target/release/sudoku-norvig-rs
All tests pass.
Solved 50 of 50 easy puzzles (avg. 0.0003 secs (3186.91 Hz), max 0.0006 secs).
Solved 95 of 95 hard puzzles (avg. 0.0010 secs (1025.89 Hz), max 0.0046 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0004 secs (2434.00 Hz), max 0.0006 secs).
Solved 99 of 99 random puzzles (avg. 0.0004 secs (2411.12 Hz), max 0.0022 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$
```

**Note: the Rust version uses much more efficient data structures.**

[original]: http://norvig.com/sudoku.html
[rustup]: https://www.rust-lang.org/tools/install
