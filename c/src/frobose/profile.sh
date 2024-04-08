g++ -o a.out frobose.cpp -lm -ggdb
perf stat -e cpu-clock,faults ./a.out
perf report
