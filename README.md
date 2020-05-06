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
