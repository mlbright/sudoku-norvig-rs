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
Solved 50 of 50 easy puzzles (avg 0.0036 secs (274.81 Hz), max 0.0078 secs).
Solved 95 of 95 hard puzzles (avg 0.0124 secs (80.50 Hz), max 0.0613 secs).
Solved 11 of 11 hardest puzzles (avg 0.0049 secs (204.76 Hz), max 0.0074 secs).
Solved 99 of 99 random puzzles (avg 0.0036 secs (278.38 Hz), max 0.0059 secs).
➜  sudoku-norvig-rs git:(master) ./sugoku
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0007 secs (1537.75 Hz), max 0.0011 secs).
Solved 95 of 95 hard puzzles (avg 0.0020 secs (489.39 Hz), max 0.0095 secs).
Solved 11 of 11 hardest puzzles (avg 0.0008 secs (1190.61 Hz), max 0.0012 secs).
Solved 99 of 99 random puzzles (avg 0.0006 secs (1605.19 Hz), max 0.0012 secs).
➜  sudoku-norvig-rs git:(master) ./target/release/sudoku-norvig-rs
All tests pass.
Solved 50 of 50 easy puzzles (avg. 0.0002 secs (4334.25 Hz), max 0.0003 secs).
Solved 95 of 95 hard puzzles (avg. 0.0008 secs (1330.86 Hz), max 0.0033 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0003 secs (3506.44 Hz), max 0.0004 secs).
Solved 99 of 99 random puzzles (avg. 0.0002 secs (4114.50 Hz), max 0.0004 secs).
➜  sudoku-norvig-rs git:(master)
```

### Linux

On a Ubuntu 16.04.6 LTS machine, I get:

```
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ python sudoku.py
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0049 secs (202.34 Hz), max 0.0070 secs).
Solved 95 of 95 hard puzzles (avg 0.0181 secs (55.25 Hz), max 0.0856 secs).
Solved 11 of 11 hardest puzzles (avg 0.0062 secs (161.54 Hz), max 0.0086 secs).
Solved 99 of 99 random puzzles (avg 0.0054 secs (183.97 Hz), max 0.0104 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./sugoku
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0010 secs (1047.70 Hz), max 0.0018 secs).
Solved 95 of 95 hard puzzles (avg 0.0031 secs (326.48 Hz), max 0.0145 secs).
Solved 11 of 11 hardest puzzles (avg 0.0012 secs (841.74 Hz), max 0.0020 secs).
Solved 99 of 99 random puzzles (avg 0.0010 secs (1015.00 Hz), max 0.0019 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./target/release/sudoku-norvig-rs
All tests pass.
Solved 50 of 50 easy puzzles (avg. 0.0003 secs (3091.42 Hz), max 0.0005 secs).
Solved 95 of 95 hard puzzles (avg. 0.0009 secs (1054.58 Hz), max 0.0044 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0005 secs (2185.45 Hz), max 0.0007 secs).
Solved 99 of 99 random puzzles (avg. 0.0003 secs (3198.06 Hz), max 0.0006 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$
```

**Note: the Rust version uses much more efficient data structures.**

[original]: http://norvig.com/sudoku.html
[rustup]: https://www.rust-lang.org/tools/install
