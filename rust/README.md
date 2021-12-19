<img src="./img/rusty_xmas.png" width="164" align="center">

# 🎄 [Advent of Code](https://adventofcode.com/)

![Language](https://badgen.net/badge/Language/Rust/orange)

## Commands

### Setup new day

```sh
# example: `./scaffold.sh 1`
./scaffold.sh <day>

# output:
# Created module `src/solutions/day01.rs`
# Created input file `src/inputs/day01.txt`
# Created example file `src/examples/day01.txt`
# Linked new module in `src/main.rs`
# Linked new module in `src/solutions/mod.rs`
# Have fun! 🎄
```

Every solution file has _unit tests_ referencing the example input file. You can use these tests to develop and debug your solution. When editing a solution file, `rust-analyzer` will display buttons for these actions above the unit tests.

### Download inputs for a day

```sh
# example: `./download.sh 1`
./download.sh <day>

# output:
# Invoking `aoc` cli...
# Loaded session cookie from "/home/foo/.adventofcode.session".
# Downloading input for day 1, 2021...
# Saving puzzle input to "/tmp/..."...
# Done!
# Wrote input to `src/inputs/day01.txt`...
# Have fun! 🎄
```


### Run solutions for a day

```sh
# example: `cargo run 1`
cargo run <day>
```

To run an optimized version for benchmarking, use the `--release` flag or the alias `cargo rr <day>`.

### Run all solutions against example input

```sh
cargo test
```
