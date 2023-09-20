Running with C code, long double until m = 10:

1.386294, 0.885223 # m = 2, p = 0.250000, a = 8
2.079442, 1.288865 # m = 3, p = 0.125000, a = 24
2.772589, 1.688379 # m = 4, p = 0.062500, a = 66
3.465736, 1.999577 # m = 5, p = 0.031250, a = 166
4.158883, 2.275249 # m = 6, p = 0.015625, a = 399
4.852030, 2.501039 # m = 7, p = 0.007812, a = 931
5.545177, 2.682406 # m = 8, p = 0.003906, a = 2129
6.238325, 2.825711 # m = 9, p = 0.001953, a = 4791
6.931472, 2.937531 # m = 10, p = 0.000977, a = 10646
Elapsed in c_30: 42.711223 secs

Running with double it became infinite at m = 9

But up to m = 8, the values matched up to 6 digits (all that was printed).

And the speed up was 9 fold:
- at m = 8, from 1.75 seconds in long double to 0.22 seconds in double.
- at m = 10, from 42.7 seconds in long double to 3.25 seconds in double.
