# Nonogram Solver

Project to learn Rust (and to hopefully get nonograms out of my head.)

Example output:
```
                 1              1
              2  2  2  2     1  2  8
              1  3  1  2  5  1  4  1  7  7  3  3  4
              2  1  5  4  2  2  2  2  4  3  8  6  6 11  7
          9 |><|><|><|██|██|██|██|██|██|██|██|██|><|><|><|
       5  7 |██|██|██|██|██|><|><|██|██|██|██|██|██|██|><|
 1  1  1  8 |██|><|██|><|██|><|██|██|██|██|██|██|██|██|><|
       7  3 |><|><|><|██|██|██|██|██|██|██|><|><|██|██|██|
    4  3  3 |><|██|██|██|██|><|><|██|██|██|><|><|██|██|██|
    2  4  2 |██|██|><|><|><|><|██|██|██|██|><|><|><|██|██|
       5  2 |><|><|><|><|><|><|██|██|██|██|██|><|><|██|██|
    2  2  2 |><|><|><|><|><|><|██|██|><|><|██|██|><|██|██|
    1  2  2 |><|><|><|><|><|><|██|><|><|><|██|██|><|██|██|
    2  1  5 |██|██|><|><|><|><|><|██|><|><|██|██|██|██|██|
       3  4 |██|██|██|><|><|><|><|><|><|><|██|██|██|██|><|
       3  6 |><|██|██|██|><|><|><|><|██|██|██|██|██|██|><|
         11 |><|><|██|██|██|██|██|██|██|██|██|██|██|><|><|
       9  1 |><|><|██|██|██|██|██|██|██|██|██|><|██|><|><|
    3  1  1 |><|██|██|██|><|><|><|><|██|><|><|><|██|><|><|
Solved in: 7.20ms
```

## Algorithm
The algorithm works on one row or column at a time, and considers all possible placements of blocks in that row or 
column. It rejects any posibilities that have blocks or voids that contradict blocks or voids currently in the working 
solution. Then it takes this set of possibilities and finds all cells that are always blocks or always voids. Cells that 
are found to be blocks in all possible solutions are updated to be blocks in the working solution, and the same occurs 
for cells that are always voids. Once this is done for one row or column, the next row or column is considered until 
the entire puzzle is solved. Each row or column will likely be visited many times.

1. Solve one vector (row or column) at a time
   1. Calculate "`gap sum`" or the total number of extra voids in the vector
      1. This is equal to the puzzle dimension minus the sum of the hint total, and one less than the hint count
         1. Eg: `15 - ((2 + 1 + 2) - (3 - 1)) = 8`
         2. So this vector would have 8 spare voids that need to be placed
   2. We know there are `gaps` equal to one plus the hint count
   3. Find all integer partitions that sum to `gap sum` with exactly `gaps` partitions
   4. Each integer partition represents a possible solution to the vector
      1. Take these possible solutions, with the hints for the vector, and generate bitvectors representing the vector
         1. Eg: `hints=[2, 1, 2]`, `gaps=[1, 2, 3, 4]` => `011001000110000`
      2. Compare these bitvectors to any known information
         1. Remove any bitvectors that have a block in a position known to be a void, and visa versa
      3. Iterate these generated bitvectors, and find all indices that are always 0, and all indicies that are always 1
         1. Indices that are always 1 mean all possible solutions for this vector has a block in that position
         2. Those which are always 0 mean all possible solutions for this vector have a void in that position
   5. Return two bitvectors: 
      1. One which has 1s in positions that always had a block
      2. The other with 1s in positions which were always voids
   6. Update the original board with any information gained- add blocks and voids matching returned bitvectors
2. Move to next row or column
   1. Iterate all rows, then all columns, then back to rows
   2. Maybe faster to alternate rows/columns more often? who cares?
3. Repeat until puzzle is solved