Pushmo Solver
-------------

This is a Python script that solves simple Pushmo levels.

 * Jumping/falling search is limited.
 * Ladders/switches are not supported yet.

The algorithm complexity: O(4^N * M)

 * N: the number of push/pull segments.
 * M: the number of possible locations.

Usage
-----

    $ python pushmo.py [-v] [-m max_depth] input.txt

Options:

 * `-v`: verbose output.
 * `-m` max_depth: the maximum depth of push/pull. (default: 3)

File format:

    ...*..
    ...DD.
    ..CCC.
    .BBBB.
    @AAAA.

 * . : blank space.
 * * : goal.
 * @ : start position.
 * A-Z : blocks.

Notice: Leave one blank at both sides of the level
(otherwise the player can't move to the side of the blocks).
