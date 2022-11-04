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

Another really neat repo is [go-sudoku][go-sudoku] by Eli Bendersky.
His code is much faster than the Norvig Python version, and was faster than this Rust version.
Then I switched to using the same u16 representation as him to represent squares of the puzzle, and now my version is faster.

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

### MacOS

On my MacBook Air (Retina, 13-inch, 2020), I get:

```
Solved 50 of 50 easy puzzles (avg. 0.0001 secs (9985.05 Hz), max 0.0001 secs).
Solved 95 of 95 hard puzzles (avg. 0.0003 secs (3208.50 Hz), max 0.0014 secs).
Solved 11 of 11 hardest puzzles (avg. 0.0001 secs (7473.58 Hz), max 0.0002 secs).
Solved 20 of 20 hardest20 puzzles (avg. 0.0010 secs (981.62 Hz), max 0.0035 secs).
Solved 1000 of 1000 hardest20x50 puzzles (avg. 0.0009 secs (1052.67 Hz), max 0.0044 secs).
Solved 87 of 87 topn87 puzzles (avg. 0.0005 secs (1889.87 Hz), max 0.0014 secs).
Solved 1 of 1 most-difficult puzzles (avg. 1.0595 secs (0.94 Hz), max 1.0595 secs).
Solved 1264 of 1264 combined puzzles (avg. 0.0016 secs (607.34 Hz), max 1.0449 secs).
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
[go-sudoku]: https://eli.thegreenplace.net/2022/sudoku-go-and-webassembly/