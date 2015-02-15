Pushmo Solver
-------------

This is a Python/Rust script that solves simple Pushmo levels.

 * Jumping/falling search is limited.
 * Ladders/switches are not supported yet.

The algorithm complexity: O(4^N * M)

 * N: the number of push/pull segments.
 * M: the number of possible locations.

Usage
-----

    (Python)
    $ python pushmo.py [-v] [-m max_depth] input.txt
    
    (Rust)
    $ rustc pushmo.rs
    $ ./pushmo [-v] [-m max_depth] input.txt

Options:

 * `-v`: verbose output.
 * `-m max_depth`: the maximum depth of push/pull. (default: 3)

File format:

    ...*..
    ...DD.
    ..CCC.
    .BBBB.
    @AAAA.

Note: Leave one blank at both sides of the level.

 * `.` : blank space.
 * `*` : goal.
 * `@` : start position.
 * `A`...`Z` : blocks.

Output:

    -- Initial state --
     ......
     ...*..
     ...DD.
     ..CCC.
     .BBBB.
     @AAAA.

    -- Move 0 --
     ......
     ...*..
     ...00.
     ..000.
     .0000.
     @0000.

    -- Move 1 --
     ......
     ...*..
     ...00.
     ..000.
     .0000.
     .33@3.

    -- Move 2 --
     ......
     ...*..
     ...00.
     ..000.
     .2@22.
     .3333.

    -- Move 3 --
     ......
     ...*..
     ...00.
     ..11@.
     .2222.
     .3333.

    -- Move 4 --
     ......
     ...*..
     ...00.
     .@222.
     .2222.
     .3333.

    -- Move 5 --
     ......
     ...*..
     ...@1.
     ..222.
     .2222.
     .3333.

    -- Move 6 --
     ......
     ...@..
     ...11.
     ..222.
     .2222.
     .3333.

    Solved in 7 steps.
