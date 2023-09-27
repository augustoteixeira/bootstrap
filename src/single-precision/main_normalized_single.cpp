#include <stdio.h>
#include <time.h>
#include <math.h>

//#include "omp.h"

// on my machine, reals are 16 bytes long, which is twice as much as double
typedef float real;

const real LOG_MULTIPLE = 1.5;
// this updates a diagonal (s = a + b) with a given past. To be defined below.
void c_modified(real p, int s, void*, void*, void*, int, real, real, real);

// there exists an infection among k fixed points
real f(real p, int k) { return 1 - powf(1 - p, k); }
// there is no infection in these k points
real n(real p, int k) { return powf(1 - p, k); }

int main(int argc, char **argv) {
  clock_t tic = clock(); // for timing purposes
  int m_min = 2; // we let p run from 2^{-m_min}, 2^{-2}, ..., 2^{-m_max} (inclusive).
  int m_max = 17;
  if (m_max < m_min) { return 1; };

  // loop through several values of p
  for (int m = m_min; m <= m_max; m++) {
    real p = powl(2, -m);
    int a_max = (int) (LOG_MULTIPLE * logf(1.0 / p) / p);

    // the probabilities will be saved with a normalizing factor
    real* log_normalizer = new real[a_max + 1];
    // so that the value M_k[a][b] = exp(log_normalizer[a + b]) current[k][a]
    for (int i = 0; i < a_max; i++) {
      log_normalizer[i] = 0.0;
    }

    // N0, N1 will rotate to progress on the calculation
    // one will be the current one, being updated and the others will be the
    // three past layers.
    // Note that we use 2 * a_max, one for each of the 2 states
    real (*N0)[a_max] = (real (*)[a_max]) malloc(sizeof(real[2][a_max]));
    real (*N1)[a_max] = (real (*)[a_max]) malloc(sizeof(real[2][a_max]));
    real (*N2)[a_max] = (real (*)[a_max]) malloc(sizeof(real[2][a_max]));
    // these pointers will be set accordingly. To fill M(i,j), we use N(i+j mod 4)
    real (*current)[a_max];
    real (*past1)[a_max];
    real (*past2)[a_max];
    // initialize everything with zeros first
    for (int k = 0; k < 2 ; k++) {
      for (int s = 0; s < a_max; s++) {
        N0[k][s] = 0; N1[k][s] = 0; N2[k][s] = 0;
      }
    }
    // update the value of the 1 x 1 boxes for the states that are reachable:
    // note that we use N2 to hold the value of the M(1,1), because 1 + 1 mod 4 = 2.
    N2[0][1] = p * n(p, 0);  // note that log_normalize is zero at 1, so no change
    N2[1][1] = p * n(p, 1);  // note that log_normalize is zero at 1, so no change

    // we now set log_normalize[3]
    log_normalizer[3] = logf(N2[0][1]);

    // we now loop over the diagonals i + j = s. As we already filled M(1, 1), we start with 3.
    for (int s = 3; s < a_max; s++) {
      // assign the pointers, depending on the role they play in the rotation
      switch (s % 3) {
      case 0:
        current = N0; past1 = N2; past2 = N1;
        break;
      case 1:
        current = N1; past1 = N0; past2 = N2;
        break;
      case 2:
        current = N2; past1 = N1; past2 = N0;
        break;
      }

      // fill the current array with an auxiliary function (implemented below)
      c_modified(p, s, current, past1, past2, a_max,
                 log_normalizer[s], log_normalizer[s - 1], log_normalizer[s - 2]);

      // calculate log_normalizer
      real max = 0.0;
      for (int l = 0; l < a_max; l++) {
        if (max < current[0][l]) { max = current[0][l]; }
      }
      log_normalizer[s + 1] = logf(max) + log_normalizer[s];

      // if we are at the last diagonal, calculate max and sum and print
      if (s == a_max - 1) {
        real sum = 0;
        for (int l = 0; l < a_max; l++) {
          sum += current[0][l];
        }
        printf("log_norm = %9.3f, ",
               log_normalizer[s]);
        printf("log(sum) = %8.3f, ",
               log(sum));
        printf("log(p) = %7.6f, -p log(sum) = %7.6f # m = %d, p = %7.6f, a = %d, max = %f, time = %f\n",
               -logf(p), -p * (logf(sum) + log_normalizer[s]), m, p, a_max, max, (float)(clock() - tic)/CLOCKS_PER_SEC);
        fflush(stdout);
      }
    }
  }
  printf("Elapsed in c_30: %f secs\n\n", (real)(clock() - tic)/CLOCKS_PER_SEC);
  return 0;
}

void c_modified(
  real p,
  int s,
  void *o,
  void *p1,
  void *p2,
  int a_max,
  real ln0,
  real ln1,
  real ln2)
{
  real convert_from_1 = expf(ln1 - ln0);
  real convert_from_2 = expf(ln2 - ln0);
  real (*output)[a_max] = static_cast<real (*)[a_max]>(o);
  real (*past1)[a_max] = static_cast<real (*)[a_max]>(p1);
  real (*past2)[a_max] = static_cast<real (*)[a_max]>(p2);
  // pragma omp parallel for
  for (int a = 1; a < s; a++) {
    int b = s - a;
    output[0][a] = f(p, b) * convert_from_1 * past1[0][a - 1]
      + p * convert_from_2 * past2[1][a - 1];
    output[1][a] = (1 - p) * f(p, a) * convert_from_1 * past1[1][a]
      + n(p, b) * output[0][a];
  }
}
