# Introduction
This repo contains solutions to the exercises from the book [Bioinformatics Algorithms: An Active Learning Approach](http://bioinformaticsalgorithms.com/). The problem numbers correspond to the problems as listed [here](http://rosalind.info/problems/list-view/?location=bioinformatics-textbook-track).

## Why Rust?
I have previously done some of these challenges in [Python](https://github.com/austinprete/BioinformaticsAlgorithms) and [Go](https://github.com/austinprete/BioAlgorithmsGo); however, over the past couple of months I've been attempting to learn Rust, and I thought it would be a worthwhile venture to reimplement these exercises in Rust (attempting to be as idiomatic as possible as a beginner to the language).

## Running solutions to exercises
A solution to a given problem can be run with `cargo run --release --bin [problem number]` where `problem number` is something like `1a`. To change the input to a given solution, simply change the corresponding `[problem number].txt` in the `test_files` directory. 