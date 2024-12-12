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

Horrible, hacky (seemingly) way to get all the diagonals from a 2d matrix.
Pretty straightforward problem, if my solution is terrible and not really
elegant heh (but it works)

## Day 05

Real thinker. Messed around with a couple solutions before this one. The gist
is to take all the pairwise comparisons given and record them for easy access
in a ranking matrix.

For the sample input, this grid would look like this (I left out all the
non-present integers, but it would be a 98 x 98 grid where all the empty spaces
are filled with `Ordering::Equal`):

```text
13 29 47 53 61 75 97
13 = > > > > > >
29 < = > > > > >
47 < < = < < > >
53 < < > = > > >
61 < < > < = > >
75 < < < < < = >
97 < < < < < < =
```

I discovered this can’t be used for a [total
order](https://en.wikipedia.org/wiki/Total_order) on the actual puzzle input
because there were cycles in the pairs given (see how [rust changed sort
implementations as of
1.81](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0.html#new-sort-implementations)).
I used `usize` for convenience (I did it with `u8` for all the pair values
originally, but kept having to cast over and over `as usize`). Didn’t notice a
performance difference, but I’m sure uses a bit more memory.

Also I Liked the `simple_grid` crate a little better than the `grid` one. Will
have to refactor that out at some point.

## Day 06

Part 1 is naïve but it works. Haven't sorted part 2 yet... **TODO**!

## Day 07

Finally used the `itertools` crate. Using the `itertools::repeat_n` function +
the `multi_cartesian_product` to get every possible combination of the
operators to be used in each equation. If any equal the desired value, add it
to the sum of possible equations.

## Day 08

**TODO**!

## Day 09

Part 1: `Vec.splice(...)` for the win! Ability to swap out ranges of values in
a vec is _very_ handy.

Part 2: Very complicated bunch of incrementing/decrementing, and tracking
position within the "disk" vec. Pretty straightforward implementation, just a
lot of juggling "pointers" (current indexes).

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
