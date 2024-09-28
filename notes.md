# Todo

## High priority



## Medium priority



## Low priority



# Functional requirements

Show a histogram of how many turing machines run exactly n steps before halting (the halting set)
Keep a tally of known infinite runners
Have a set of unknown machines
Only run the ones that are unknown, and stop them as soon as they are known
Build an infinite loop detector
Create a machine visualizer


# User interface

See histogram
See tallies of infinite runners, unknowns, and halters
- halters broken up into numbers of steps
Get indices of infinite runners
Get indices of machines that halt after n steps
Get indices of unknown machines
See periodicity leaderboard among infinite runners
- local periodicity leaderboard
- asymptotic periodicity leaderboard
See steps leaderboard among halters
See 1's leaderboard among halters


# Transferred data

tallies/indices of infinites, unknowns, halters 
tallies/indices of halters with number of steps taken
periodicity heap (among infinites)
steps taken heap (among halters)
numbers of 1's heap (among halters)


# System design

Create an array of turing machines
Keep a set of running machines
Keep a set of halted machines
Keep a set of infinite runners
Infinite loop detection
- local periodicity 
- asymptotic periodicity
Chunk removal optimization
- if a machine's result is determined by a set of properties, skip all machines that share that property
Be able to determine a turing machine's transition functions based on its index


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

After a node is generated, it will be classified as halting, unknown, or infinite.
- All halting or infinite nodes are leaf nodes.
- All unknown nodes are internal nodes


## Busy beaver algorithm

Choose a constant number of steps we give a beaver in one "batch"

Each beaver must traverse its states in numerical order to prevent [distinct but functionally equivalent] beavers from co-existing.

For each #states, there is a set of beaver id's that have that many states.

We want to prioritize beavers that have a smaller number of states.
The dedicated computational resources towards a beaver will be proportional to the product of the following factors:
- (1/2) ^ #states
- 1 / (#beavers w/ same #states)

How to implement beaver frequency:
- use a fast queue of beavers for each unique #states
- determine #states to dedicate compute to using the 2-adic
  valuation of the total number of beaver runs completed.
  - When all beavers with up to n states have been categorized,
    subtract n and add 1 to the 2-adic valuation to get the next
    [#states equivalence class] to dedicate compute to.
  - use i.trailing_zeros() - n + 1
