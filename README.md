# Advent of Code 2020

This repository contains the solutions for all exercises of Advent of Code 2020 Edition.

## Run

Please to clone first, along with aoc2020 repo, the `aoc_helper` crate [here](https://github.com/k0pernicus/aoc_helper),
as each exercise will need this dependency to be compiled.

`cargo run` will look for a file called `input.txt` that contains your own input.  
If your input is in another file, please to give, as argument, the input filename.

Please to use the **release** version of each binary, in order to get significant performance improvements 
for each exercise: `cargo build --release`.

Each exercise binary is a CLI - if you want to know the argument(s) to provide: `./target/release/ex<> --help`.
