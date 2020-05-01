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
Solved 50 of 50 easy puzzles (avg 0.0037 secs (268.09 Hz), max 0.0062 secs).
Solved 95 of 95 hard puzzles (avg 0.0129 secs (77.67 Hz), max 0.0626 secs).
Solved 11 of 11 hardest puzzles (avg 0.0049 secs (202.31 Hz), max 0.0074 secs).
Solved 99 of 99 random puzzles (avg 0.0038 secs (259.94 Hz), max 0.0064 secs).
➜  sudoku-norvig-rs git:(master) go run sugoku.go
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0007 secs (1456.49 Hz), max 0.0012 secs).
Solved 95 of 95 hard puzzles (avg 0.0020 secs (495.40 Hz), max 0.0095 secs).
Solved 11 of 11 hardest puzzles (avg 0.0008 secs (1315.79 Hz), max 0.0010 secs).
Solved 99 of 99 random puzzles (avg 0.0006 secs (1543.43 Hz), max 0.0015 secs).
➜  sudoku-norvig-rs git:(master) ./target/release/sudoku-norvig-rs
Solved 50 of 50 easy puzzles (avg. 0.0008 secs (1232.91 Hz), max 0.0013 secs).
Solved 95 of 95 hard puzzles (avg. 0.0021 secs (483.56 Hz), max 0.0094 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0009 secs (1053.19 Hz), max 0.0016 secs).
Solved 99 of 99 random puzzles (avg. 0.0008 secs (1195.16 Hz), max 0.0014 secs).
➜  sudoku-norvig-rs git:(master)
```

### Linux

On a Ubuntu 16.04.6 LTS machine, I get:

```
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ python sudoku.py
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0048 secs (209.05 Hz), max 0.0060 secs).
Solved 95 of 95 hard puzzles (avg 0.0175 secs (57.16 Hz), max 0.0843 secs).
Solved 11 of 11 hardest puzzles (avg 0.0063 secs (158.94 Hz), max 0.0092 secs).
Solved 99 of 99 random puzzles (avg 0.0052 secs (193.25 Hz), max 0.0070 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./sugoku
All tests pass.
Solved 50 of 50 easy puzzles (avg 0.0010 secs (1035.13 Hz), max 0.0019 secs).
Solved 95 of 95 hard puzzles (avg 0.0029 secs (349.68 Hz), max 0.0126 secs).
Solved 11 of 11 hardest puzzles (avg 0.0012 secs (847.98 Hz), max 0.0021 secs).
Solved 99 of 99 random puzzles (avg 0.0009 secs (1145.66 Hz), max 0.0014 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$ ./target/release/sudoku-norvig-rs
Solved 50 of 50 easy puzzles (avg. 0.0005 secs (2073.90 Hz), max 0.0010 secs).
Solved 95 of 95 hard puzzles (avg. 0.0013 secs (791.31 Hz), max 0.0059 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0005 secs (1855.73 Hz), max 0.0007 secs).
Solved 99 of 99 random puzzles (avg. 0.0005 secs (2152.64 Hz), max 0.0005 secs).
administrator@ECS-4b01ba5b:~/sudoku-norvig-rs$
```


[original]: http://norvig.com/sudoku.html
[rustup]: https://www.rust-lang.org/tools/install
