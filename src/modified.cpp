// Halide rule 30

// On linux, you can compile and run it like so:
// g++ lesson_07*.cpp -g -std=c++17 -I <path/to/Halide.h> -I <path/to/tools/halide_image_io.h> -L <path/to/libHalide.so> -lHalide `libpng-config --cflags --ldflags` -ljpeg -lpthread -ldl -o lesson_07
// LD_LIBRARY_PATH=<path/to/libHalide.so> ./lesson_07

// On os x:
// g++ lesson_07*.cpp -g -std=c++17 -I <path/to/Halide.h> -I <path/to/tools/halide_image_io.h> -L <path/to/libHalide.so> -lHalide `libpng-config --cflags --ldflags` -ljpeg -o lesson_07
// DYLD_LIBRARY_PATH=<path/to/libHalide.dylib> ./lesson_07

//#include "Halide.h"
#include <stdio.h>
#include <time.h>
#include <math.h>

// using namespace Halide;

// Support code for loading pngs.
//#include "halide_image_io.h"
//#include "rule_30_halide.h"

//using namespace Halide::Tools;
// using namespace Halide;


void c_modified(double p, int s, double output[], double past1[], double past2[], double past3[], int a_max);

bool IsPowerOfTwo(ulong x)
{
  return (x != 0) && ((x & (x - 1)) == 0);
}

double f(double p, int k) {
  return 1 - pow(1 - p, k);
}

double n(double p, int k) {
  return pow(1 - p, k);
}

int main(int argc, char **argv) {
  {
    clock_t tic = clock();
    for (int m = 1; m < 8; m ++) {
      double p = pow(2, -m);
      int a_max = 4*log(1/p)/p;
      double* N0 = new double[7 * a_max];
      double* N1 = new double[7 * a_max];
      double* N2 = new double[7 * a_max];
      double* N3 = new double[7 * a_max];
      double* current;
      double* past1;
      double* past2;
      double* past3;

      for (int j = 0; j < 7 * a_max; j++) {
        N0[j] = 0; N1[j] = 0; N2[j] = 0; N3[j] = 0;
      }
      N2[1] = p;

      tic = clock();
      for (int s = 3; s < a_max; s++) {
        switch (s % 4) {
          case 0:
            current = N0; past1 = N3; past2 = N2; past3 = N1;
            break;
          case 1:
            current = N1; past1 = N0; past2 = N3; past3 = N2;
            break;
          case 2:
            current = N2; past1 = N1; past2 = N0; past3 = N3;
            break;
          case 3:
            current = N3; past1 = N2; past2 = N1; past3 = N0;
            break;
        }
        c_modified(p, s, current, past1, past2, past3, a_max);
        if (s == a_max - 1) {
          double sum = 0;
          for (int l = 0; l < a_max; l++) {
            sum += current[l];
          }
          printf("m = %d, p = %f, N = %d, log(sum{M(x, N-x)}) = %f, -p*lm = %f, -log(middle) = %f\n",
                 m, p, a_max, log(sum), -p * log(sum), -log(current[s/2]));
        }
      }
      delete N0;
      delete N1;
      delete N2;
      delete N3;
    }
    printf("Elapsed in c_30: %f secs\n", (double)(clock() - tic)/CLOCKS_PER_SEC);
  }
  return 0;
}

void c_modified(
  double p,
  int s,
  double output[],
  double past1[],
  double past2[],
  double past3[],
  int a_max)
{
  for (int a = 1; a < s; a++) {
    int b = s - a;
    output[0 * a_max + a] =
      p * (past2[1 * a_max + a - 1] + past2[5 * a_max + a - 1] + past2[6 * a_max + a - 1])
      + f(p, b) * past1[0 * a_max + a - 1];
    output[1 * a_max + a] = (1 - p) * f(p, a) * past1[1 * a_max + a]
      + n(p, b) * output[0 * a_max + a] + p * past2[2 * a_max + a - 1];
    output[5 * a_max + a] = (1 - p) * f(p, a) * past1[5 * a_max + a]
      + p * past2[4 * a_max + a - 1];
    // be careful!!! Here we are overflowing on purpose for a - 2,
    // but this seems fine for testing
    output[6 * a_max + a] = (1 - p) * n(p, b) * past1[6 * a_max + a - 1]
      + p * p * past3[3 * a_max + a - 2];
    output[2 * a_max + a] = (1 - p) * f(p, b) * past1[2 * a_max + a - 1]
      + p * (1 - p) * past2[3 * a_max + a - 1]
      + (1 - p) * n(p, a) * output[1 * a_max + a]
      + (1 - p) * n(p, b) * output[6 * a_max + a];
    output[4 * a_max + a] = (1 - p) * f(p, b) * past1[4 * a_max + a - 1]
      + p * (1 - p) * past2[3 * a_max + a - 1]
      + (1 - p) * n(p, a) * output[5 * a_max + a];
    output[3 * a_max + a] = (1 - p) * (1 - p) * f(p, a) * past1[3 * a_max + a]
      + (1 - p) * n(p, b) * output[2 * a_max + a]
      + (1 - p) * n(p, b) * output[4 * a_max + a];
  }
}
