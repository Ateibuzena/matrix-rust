# Exercise 00 — Complexity Analysis

## Time Complexity

All six operations (`add`, `sub`, `scl` for vectors and matrices) have a time complexity of **O(n)**.

Each operation iterates exactly once over the elements of the input vector or matrix.

- Vector operations perform one operation per coordinate.
- Matrix operations perform one operation per matrix element.
- Each iteration performs a constant-time arithmetic operation.

For matrices, `n` represents the total number of elements (`rows × cols`).

Therefore, the time complexity is O(n).

## Space Complexity

All six operations have an auxiliary space complexity of **O(1)**.

They modify the existing data in place using mutable references (`&mut self`).

No additional vectors, matrices, or collections are allocated during these operations.

Only a constant amount of auxiliary memory is required for variables and iteration.

Therefore, the auxiliary space complexity is O(1), which satisfies the maximum O(n) constraint.

## Mathematical Functions

No mathematical library functions are used. The implementation relies exclusively on basic arithmetic operations: addition, subtraction and multiplication.