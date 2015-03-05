# Algorithms Cheat Sheet

An **algorithm** is a clearly specified set of simple instructions to be followed to solve a problem.

## Typical Growth Rates

| Function | Name        |
|----------|-------------|
| c        | Constant    |
| logN     | Logarithmic |
| log^2 N  | Log-square  |
| N        | Linear      |
| NlogN    |             |
| N^2      | Quadratic   |
| N^3      | Cubic       |
| 2^N      | Exponential |

## Big-O General Rules

**Rule 1 - _for_ loops**
> The running time of a for loop is at most the running time of the statements inside the for loop (including tests) times the number of iterations.

**Rule 2 - Nested loops**
> Analyze these inside out. The total running time of a statement inside a group of nested loops is the running time of the statement multiplied by the product of the sizes of all the loops.

**Rule 3 - Consecutive Statements**
> These just add (which means that the maximum is the one that counts).

**Rule 4 - if/else**
> For the fragment
```
if(condition)
    S1
else
    S2
```
The running time of an if/else statement is never more than the running time of the test plus the larger of the running times of S1 and S2.

**LogN Running Time**
> An algorithm is O(logN) if it takes contant (O(1)) time to cut the problem size by a faction (which is usually 1/2).

Ex. Binary Search

## Terms

**Online Algorithm** - An algorithm that requires only constant space and runs in linear time.


## References

1. [Data Structures And Algorithm Analysis in Java 3rd Edition by Mark Allen Weiss](http://www.amazon.com/Data-Structures-Algorithm-Analysis-Java/dp/0132576279/ref=sr_1_1?ie=UTF8&qid=1425517283&sr=8-1&keywords=Data+Structures+and+Algorithms+analysis+in+java+mark+allen)
2. [Big-O Cheat Sheet](http://bigocheatsheet.com/)