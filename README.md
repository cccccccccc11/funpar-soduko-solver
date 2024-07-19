## Overview
The project aims to solve a Sudoku Puzzle using different solutions and compare their performances. We made 4 versions of them, 2 using parallel coding, and 2 using sequential using DFS and backtracking algorithms.

## Results
After several tests, we conclude that the fastest method is crossbeam as we can modify our code in more detail compared to rayon which only needs minimal changes for it to work. However, crossbeam can be more complex and take more time to code than other solutions.

## Problem
Our code works fine for the 9x9 grid, but when we try to make a 16x16 grid there seems to be a problem. It seems like our approach doesn’t work with big grids; the program can read the table but seems to take a long time and gets stuck while solving all the empty spaces.

## Conclusion
Why is parallel faster than non-parallel?

In a parallel program, multiple tasks can be executed at the same time, which can significantly reduce the overall workload compared to the sequential version. While sequential code uses a single thread, parallel code utilizes multiple CPU cores more efficiently, leading to better performance.

## Contributors
- Chinanard Sathiseth 6481366
- Nontanapong Thanasetkorn 6481237