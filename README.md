# bootstrap percolation

This repository includes three implementations of modified and Froböse Bootstrap percolation using a new algorithm developed by Hartarsky and Teixeira.

# python

    sudo apt install python3-sklearn
    sudo apt install python3-notebook

# rust

    cargo run
    
# c

Uses simple floating points, reducing precision and limiting size of simulation.
To mitigate this, values in the same diagonal are normalized, but this is not perfect either.
This code does not scale to very small values of `p`.

In `c/src/frobose/` run `script`.

Parameters can be modified in the source code.
