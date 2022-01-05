# sudoku-norvig-rs

This repository contains a Rust version of [Peter Norvig's "Solve Every Sudoku Puzzle"][original].

# The rabbit hole

Sudoku was said to be a "denial of service attack on the brain" (apparently by [Ben Laurie][laurie], of Apache `httpd` fame).
Writing a Sudoku solver is an even bigger denial of service attack on the brain.
However, as a side effect of writing this library, I was able to learn some software development tricks in Rust, Go, and Python.

I improved Peter Norvig's version slightly by using Python lists instead of dictionaries.
This turns out not to complicate the code too much while improving performance.

Then I turned to Go when it was my favorite language.
This resulted in significantly better performance.

Rust is more performant than Go, although on MacOS this is only true when using the [jemalloc memory allocator][jemalloc].
The current version does _not_ use the `jemalloc` allocator, to keep things simple.

Making a fast Sudoku solver is more complicated than perhaps meets the eye.
Much has been [written][attractivechaos], and there's a devoted gang of solver nerds that have cut their teeth on this problem, including [the current prime minister of Singapore][singaporepm].
The fastest solver I could find was [tdoku][tdoku] by [t-dillon]. 
His [writeup][math] is a very deep dive.

The fastest Rust version I found was https://github.com/Emerentius/sudoku.
My version is not nearly as fast because it sticks to Norvig's basic approach.

The coolest Sudoku thing ever is the [augmented reality Sudoku solver][ar] that makes uses of the fast Rust solver by [Emerentius][emerentius].

## Run

Install Rust with [rustup][rustup], which installs `cargo`.
Then run:

```
cargo run --release --example bench
```

This will compile the library and run it against a set of [puzzles](/puzzles).

## Library

Use this code as a Rust library.
It currently doesn't reside in [crates.io][cratesio], so you'll need to include it in your `Cargo.toml` file like:

```toml
[dependencies]
sudoku = { git = "https://github.com/mlbright/sudoku-norvig-rs" }
 ```

## Performance

Run:

```bash
python3 sudoku.py # For the Python benchmarks
cargo run --release --example bench # For the Rust benchmarks
```

Below are old benchmarks, but the conclusions you can draw from them are probably still valid.

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

[original]: http://norvig.com/sudoku.html
[rustup]: https://www.rust-lang.org/tools/install
[jemalloc]: https://github.com/gnzlbg/jemallocator
[emerentius]: https://github.com/Emerentius
[attractivechaos]: https://attractivechaos.wordpress.com/2011/06/19/an-incomplete-review-of-sudoku-solver-implementations/
[laurie]: https://en.wikipedia.org/wiki/Ben_Laurie
[math]: https://t-dillon.github.io/tdoku/
[t-dillon]: https://github.com/t-dillon
[tdoku]: https://github.com/t-dillon/tdoku
[fastest-rust]: https://github.com/Emerentius/sudoku
[ar]: https://github.com/ColinEberhardt/wasm-sudoku-solver
[singaporepm]: https://en.wikipedia.org/wiki/Lee_Hsien_Loong
[cratesio]: https://crates.io
