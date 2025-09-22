# Water Sort Solver
## Introduction
I created this to solve those water/marble/whatever sort puzzles. There are three structs: liquid, vial, and solver. Liquid is the color and amount which you can use to create. vial is the container for the liquid. Finally solver is what holds original state and solves to the puzzle.

## Example
In main.rs I have a sample version of a problem I was solving for a game. I put the liquids in reverese order in the vectors so I reveresed it in the code with iter.rev() because that is how my brain worked but you could do whatever way you wanted. the vector is meant to be [bottom -> top]

## Algorithm
The algorithm is using DFS for a graph keeping track of the game state as the nodes. I chose this because I'm looking for a solution not the shortest so It's probably better than BFS because the answer will most likely be one of the longer branch. I haven't done the math to prove this though.
