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
- run infinite loop detection
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

### Periodic infinite loop detection

Let history = Vec<(u8, u8)> // vector of (state, symbol) pairs
Let v = history[history.len() - 1]
Let offsets = Vec<HashSet<usize>> // each usize is an offset
Let offset = 0

While offset * offsets.len() < history.len() {
  If history[history.len() - offset] == v:
  - always add offset to offsets[0]
  - add offset to offsets[1] if offset / 2 is in offsets[0]
  - add offset to offsets[2] if offset / 3 is in offsets[0] and 2 * offset / 3 is in offsets[1]
  - add offset to offsets[n] if [ (i + 1) * offset / (n + 1) is in offsets[i] ] for all i in (0..n).reverse()

  **Iterate i in decreasing order as an optimization because the offset hashsets are smaller for higher i, allowing for early exit
}


### Aperiodic infinite loop detection
