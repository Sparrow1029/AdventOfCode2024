# Advent of Code 2024

Solutions for the Advent of Code 2024 challenges in Rust

## Day 01

Pretty simple. Be nice to come up with a macro to implement all the match
cases instead of manually handling all of them for running test/solve.
Bet there's a more elegant way to do it, but whatever! Got it to work.

## Day 02

Opted for a `lazy_static!`-ally created `HashMap` of day->solve function
pointers. We'll see how gross that looks as the days go on...

Overall, spent more time than I should have on this simple one. Return of the
`nom` crate for parsing puzzle inputs, and I tried to be very functional in
the approach to testing the sequences.

## Day 03

Got stuck for a while because of how the `regex` crate's `captures_iter` works.
Ended up with an okay solution.

## Day 04

**TODO**!

## Day 05

**TODO**!

## Day 06

**TODO**!

## Day 07

**TODO**!

## Day 08

**TODO**!

## Day 09

**TODO**!

## Day 10

Decided to use `petgraph`. The nodes are weighted with the values from the
grid, and the only edges that exist are between nodes that have a weight
difference of one. The graph is also directed in that only paths from 0->9 have
connections. I'm certain that a depth-first-search traversal of the grid would
have worked just fine and saved some work... but hey it works.

## Day 11

Original solution was to use `VecDeque<u64>`, but that proved too naïve for day 2.
Cribbed off answers found online using `HashMap` tracking counts of values
present in the vector. Forgot about that trick, though it's been the optimal
solution for exponential growth problems in the past.
