#include <stdio.h>
#include <time.h>
#include <math.h>

#include "omp.h"

// on my machine, reals are 16 bytes long, which is twice as much as double
typedef float real;

const real LOG_MULTIPLE = 1.5;
// this updates a diagonal (s = a + b) with a given past. To be defined below.
void c_modified(real p, int s, void*, void*, void*, void*, void*, int, real, real, real, real, real, real*);

// there exists an infection among k fixed points
real f(real p, int k) { return 1 - powf(1 - p, k); }
// there is no infection in these k points
real n(real p, int k) { return powf(1 - p, k); }

int main(int argc, char **argv) {
  clock_t tic = clock(); // for timing purposes
  int m_min = 2; // we let p run from 2^{-m_min}, 2^{-2}, ..., 2^{-m_max} (inclusive).
  int m_max = 11;
  if (m_max < m_min) { return 1; };

  FILE *fpt;
  fpt = fopen("report.csv", "w+");

  // loop through several values of p
  for (int m = m_min; m <= m_max; m++) {
    real p = powl(2, -m);
    int a_max = (int) (LOG_MULTIPLE * logf(1.0 / p) / p);

    // the probabilities will be saved with a normalizing factor
    real* log_normalizer = new real[a_max + 3]; // here we add 3 because of shifting below and +1 later
    // so that the value M_k[a][b] = exp(log_normalizer[a + b]) current[k][a]
    for (int i = 0; i < a_max + 3; i++) {
      log_normalizer[i] = 0.0;
    }

    log_normalizer += 2; // here we shift it to obtain two allow access to [-1]

    // N0, N1 will rotate to progress on the calculation
    // one will be the current one, being updated and the others will be the
    // three past layers.
    // Note that we use 2 * a_max, one for each of the 2 states
    real (*N0)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
    real (*N1)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
    real (*N2)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
    real (*N3)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
    real (*N4)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
    // these pointers will be set accordingly. To fill M(i,j), we use N(i+j mod 4)
    real (*current)[a_max];
    real (*past1)[a_max];
    real (*past2)[a_max];
    real (*past3)[a_max];
    real (*past4)[a_max];
    // initialize everything with zeros first
    for (int k = 0; k < 2 ; k++) {
      for (int s = 0; s < a_max; s++) {
        N0[k][s] = 0; N1[k][s] = 0; N2[k][s] = 0; N3[k][s] = 0; N4[k][s] = 0;
      }
    }
    // update the value of the 1 x 1 boxes for the states that are reachable:
    // note that we use N2 to hold the value of the M(1,1), because 1 + 1 mod 4 = 2.
    N2[0][1] = p * n(p, 0); // note that log_normalize is zero at 1, so no change
    N2[1][1] = p * n(p, 1); // origin infected, east buffered
    N2[2][1] = p * n(p, 2); // origin infected, east and north buffered
    N2[3][1] = p * n(p, 3); // origin infected, east, west and north buffered
    N2[4][1] = 0;
    N2[5][1] = 0;
    N2[6][1] = 0;

    // we now set log_normalize[3]
    log_normalizer[3] = logf(N2[0][1]);

    // we now loop over the diagonals i + j = s. As we already filled M(1, 1), we start with 3.
    for (int s = 3; s < a_max; s++) {
      // assign the pointers, depending on the role they play in the rotation
      switch (s % 5) {
      case 0:
        current = N0; past1 = N4; past2 = N3; past3 = N2; past4 = N1;
        break;
      case 1:
        current = N1; past1 = N0; past2 = N4; past3 = N3; past4 = N2;
        break;
      case 2:
        current = N2; past1 = N1; past2 = N0; past3 = N4; past4 = N3;
        break;
      case 3:
        current = N3; past1 = N2; past2 = N1; past3 = N0; past4 = N4;
        break;
      case 4:
        current = N4; past1 = N3; past2 = N2; past3 = N1; past4 = N0;
        break;
      }

      // fill the current array with an auxiliary function (implemented below)
      c_modified(p, s, current, past1, past2, past3, past4, a_max,
                 log_normalizer[s], log_normalizer[s - 1], log_normalizer[s - 2], log_normalizer[s - 3], log_normalizer[s - 4], log_normalizer + (s + 1));

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
        printf("log(p) = %7.6f, -p log(sum) = %7.6f # m = %d, p = %7.6f, a = %d, time = %f\n",
               -logf(p), -p * (logf(sum) + log_normalizer[s]), m, p, a_max, (float)(clock() - tic)/CLOCKS_PER_SEC);
        fflush(stdout);
        fprintf(fpt, "%14.7f, %14.7f\n", -p * (logf(sum) + log_normalizer[s]), p);
      }
    }
  }
  printf("Elapsed in c_30: %f secs\n\n", (real)(clock() - tic)/CLOCKS_PER_SEC);
  fclose(fpt);
  return 0;
}

void c_modified(
  real p,
  int s,
  void *o,
  void *p1,
  void *p2,
  void *p3,
  void *p4,
  int a_max,
  real ln0,
  real ln1,
  real ln2,
  real ln3,
  real ln4,
  real* ln_next)
{
  real convert_from_1 = expf(ln1 - ln0);
  real convert_from_2 = expf(ln2 - ln0);
  real convert_from_3 = expf(ln3 - ln0);
  real convert_from_4 = expf(ln4 - ln0);
  real (*output)[a_max] = static_cast<real (*)[a_max]>(o);
  real (*past1)[a_max] = static_cast<real (*)[a_max]>(p1);
  real (*past2)[a_max] = static_cast<real (*)[a_max]>(p2);
  real (*past3)[a_max] = static_cast<real (*)[a_max]>(p3);
  real (*past4)[a_max] = static_cast<real (*)[a_max]>(p4);

  // running max for log_normalizer
  real max = 0.0;

  //pragma omp parallel for
  for (int a = 1; a < s; a++) {
    // order 0, 1, 6, 5, 4, 2, 3
    int b = s - a;
    output[0][a]
      = f(p, b) * convert_from_1 * past1[0][a - 1]         // 0 -> 0
      + f(p, a) * p * convert_from_2 * past2[1][a - 1]     // 1 -> 0
      + f(p, b) * p * p * convert_from_3 * past3[2][a - 2] // 2 -> 0
      + (4 * f(p, a) * p * p * p * n(p, 1) + p*p*p*p) * convert_from_4 * past4[3][a - 2] // 3 -> 0
      + f(p, b) * p * p * convert_from_3 * past3[4][a - 2] // 4 -> 0
      + f(p, a) * p * convert_from_2 * past2[5][a - 1]     // 5 -> 0
      + f(p, b) * p * convert_from_2 * past2[6][a - 1];    // 6 -> 0
    // trim small values
    if (output[0][a] < 0.0000001) { output[0][a] = 0.0; }
    // update running max
    if (max < output[0][a]) { max = output[0][a]; }
    output[1][a]
      = n(p, b) * output[0][a]                                    // 0 -> 1
      + f(p, a) * (1 - p) * convert_from_1 * past1[1][a]          // 1 -> 1
      + f(p, b) * p * (1 - p) * convert_from_2 * past2[2][a - 1]  // 2 -> 1
      + f(p, a) * p * p * (1 - p) * (1 - p) * convert_from_3 * past3[3][a - 1];  // 3 -> 1
    // trim small values
    if (output[1][a] < 0.0000001) { output[1][a] = 0.0; }
    output[6][a]
      = f(p, a) * p * p * (1 - p) * (1 - p) * convert_from_3 * past3[3][a - 2]  // 3 -> 6
      + f(p, b) * (1 - p) * convert_from_1 * past1[6][a - 1];     // 6 -> 6
    // trim small values
    if (output[6][a] < 0.0000001) { output[6][a] = 0.0; }
    output[5][a]
      = f(p, a) * p * p * (1 - p) * (1 - p) * convert_from_3 * past3[3][a - 1]  // 3 -> 5
      + f(p, b) * p * (1 - p) * convert_from_2 * past2[4][a - 1]  // 4 -> 5
      + f(p, a) * (1 - p) * convert_from_1 * past1[5][a];         // 5 -> 5
    // trim small values
    if (output[5][a] < 0.0000001) { output[5][a] = 0.0; }
    output[4][a]
      = f(p, a) * p * (1 - p) * (1 - p) * convert_from_2 * past2[3][a - 1]  // 3 -> 4
      + f(p, b) * (1 - p) * convert_from_1 * past1[4][a - 1]      // 4 -> 4
      + n(p, a) * output[5][a];                                   // 5 -> 4
    // trim small values
    if (output[4][a] < 0.0000001) { output[4][a] = 0.0; }
    output[2][a]
      = n(p, a) * output[1][a]                                    // 1 -> 2
      + f(p, b) * (1 - p) * convert_from_1 * past1[2][a - 1]      // 2 -> 2
      + f(p, a) * p * (1 - p) * (1 - p) * convert_from_2 * past2[3][a - 1]  // 3 -> 2
      + n(p, b) * output[6][a];                                   // 6 -> 2
    // trim small values
    if (output[2][a] < 0.0000001) { output[2][a] = 0.0; }
    output[3][a]
      + n(p, b) * output[2][a]                                     // 2 -> 3
      + f(p, a) * (1 - p) * (1 - p) * convert_from_1 * past1[3][a] // 3 -> 3
      + n(p, b) * output[4][a];                                    // 4 -> 3
    // trim small values
    if (output[3][a] < 0.0000001) { output[3][a] = 0.0; }
  }
  //printf("\ns = %d, amax = %d, max = %15.10f\n", s, a_max, max);
  *ln_next = logf(max) + ln0;
}
