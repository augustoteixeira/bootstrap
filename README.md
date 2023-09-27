# installation

    sudo apt install libgmp-dev
    sudo apt install libmpfr-dev

# for python

    sudo apt install python3-sklearn
    sudo apt install python3-notebook

# bootstrap


# TODO

## Modified Bootstrap

- write down transitions in a convenient way
- test exp(-INFTY) for boundary condition (non-Halide)
- c non-optimized code for verification
  - allocates square
  - uses simple boundary condition
  - uses large floating point types
  - keep the probabilities instead of their logarithms
- check minimum floating requirements
  - https://en.wikipedia.org/wiki/Half-precision_floating-point_format
- implement Halide for modified
- check if implementations match
- try to optimize Halide and see improvement against c
  - see if logs in Halide vectorize
- try to extract first term from Halide

## Full bootstrap

- show results of modified to Ivailo
- pass a format for Ivailo to fill
