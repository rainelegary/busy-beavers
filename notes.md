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
- The root node's children are (0, 0) -> (..x, ..y, ..z) | halt

After a node is generated, it will be classified as halting, unknown, or infinite.
- All halting or infinite nodes are leaf nodes.
- All unknown nodes are internal nodes

We will want to avoid generating mirrored branches of the tree.
If it is feasible to calculate the amount of times a pattern is
mirrorable and count it with that multiplicity, we can avoid
duplicating the calculation for all of its mirrors.


## Chunk removal

Chunks are periodic windows that contain blacklisted sections





