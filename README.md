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
