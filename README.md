# bootstrap percolation

This repository includes three implementations of modified and Froböse Bootstrap percolation using a new algorithm developed by Hartarsky and Teixeira.

# taichi

This is the fastest code, as it runs on the GPU (needs drivers to be installed, tested with CUDA).

Enter the taichi folder and install a Python virtual environment:

    python3 -m virtualenv vir

Activate it (in Bash):

    source ./vir/bin/activate

Install the `requirements.txt`:

    pip install -r requirements.txt

Execute either `modified.py` or `frobose.py`.
If no GPU is found, it falls back to CPU.

# rust

Install Rust and Cargo:

    https://www.rust-lang.org/tools/install

To execute:

    cargo run

In order to change the way to store the floating points, look at the first comments in `main.rs`.
This is also the right file to alter sizes and parameters.

# c

Install gcc. In Ubuntu/Debian, for instance:

    sudo apt update
    sudo apt install build-essential

Uses simple floating points, reducing precision and limiting size of simulation.
To mitigate this, values in the same diagonal are normalized, but this is not perfect either.
This code does not scale to very small values of `p`.

In `c/src/frobose/` run `script`.

Parameters can be modified in the source code.

# plotting and fitting

Two simple Python scripts have been added to aid in this: `4fit.py` and `plotting.py`.
