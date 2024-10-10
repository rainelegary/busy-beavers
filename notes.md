# Todo

## High priority


## Medium priority


## Low priority



# User interface

Display types
- tally
- list beavers
- histogram of common low-end scores
- leaderboard of rare high-end score

Beaver statistics
- halters
  - lifetime
  - max distance from origin
  - #1's
- infinites (dormant / propagating)
  - pre-periodic lifetime
  - pre-periodic max distance from origin
  - pre-periodic #1's
  - periodicity

Aggregate statistics
- #unknown
- #halting
- #dormant
- #propagating

Querying to see a leaderboard among a given beaver's children

Single-beaver spectating

# Implementation

## Transition function enumeration

transition function:
(state, symbol) -> (state, symbol, delta)

Enumerations are a number where each "digit" has a different amount of possible values

Digit 1 is the least significant digit
Digit n has [(#states | min(n + 1, max_states)) * (#symbols | min(n + 1, max_symbols)) * (#deltas | 2 * max_delta + 1)] possible values

Each digit defines what the transition function should do next once it reaches a previously unvisited (state, symbol) pair.
n represents the number of (state, symbol) pairs that have been visited, including the one that it just being visited now.

If we are on digit m where m is greater than the number of digits, the transition function assumes it must go to a halting state as its next move,
meaning the resulting machine will halt.

## Breadth-first tree search of turing-machine space

A node is defined by its parent + some additional information:
- the outputted (x, y, z) | halt on its first not-yet defined (state, symbol) pair

Everything starts from empty root node
- The root node's children are (0, 0) -> (..x, ..y, ..z)


## Busy beaver algorithm

For each #states, there is a set of running beaver id's that have that many states.
Each beaver must traverse its states in numerical order to prevent [distinct but functionally equivalent] beavers from co-existing.
Run the beaver for a constant number of steps.

if a beaver halts
- move it from the running category to the halting category
- generate children and place them in the queues for their respective [#states equivalence classes]

If the beaver does not halt
- run infinite loop detection if #steps has reached or passed a new power of 2.
  - classify it as dormant or propagating if possible

If the beaver runs forever
- move it from the running category to the dormant or propagating category

We want to prioritize beavers that have a smaller number of states.
The dedicated computational resources towards a beaver will be proportional to the product of the following factors:
- (1/2) ^ #states
- 1 / #[beavers w/ same #states]

How to implement beaver frequency:
- use a fast queue of beavers for each unique #states
- determine #states to dedicate compute to using the 2-adic
  valuation of the total number of beaver runs completed.
  - When all beavers with up to n states have been categorized,
    subtract n and add 1 to the 2-adic valuation to get the next
    [#states equivalence class] to dedicate compute to.
  - use i.trailing_zeros() - n + 1

## Infinite loop detection

### Rolling prime factorization algorithm


### NEW periodic infinite loop detection

let h = history
let hl = h.len()
let li = hl - 1

for p in 1..=hl / 2 {
  for o in 0..p {
    if h[li] != h[li - p] {
      continue outer loop;
    }
  }
  
  loop test
}

#### Loop test

- if same location, test passes and classified as dormant
- if different location and new outer bound reached on tape at some point in the loop, test passes and classified as propagating
- otherwise, test fails
