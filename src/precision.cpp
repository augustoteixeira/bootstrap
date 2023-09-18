#include <gmp.h>
#include <mpfr.h>
#include <stdio.h>
#include <time.h>
#include <math.h>

//#include "omp.h"

typedef long double real;

const real LOG_MULTIPLE = 3;
// there exists an infection among k fixed points
real f(real p, int k) { return 1 - powl(1 - p, k); }
// there is no infection in these k points
real n(real p, int k) { return powl(1 - p, k); }
// there is no infection in these k points
void nm(mpfr_t p, int k) {
  mpfr_ui_sub(p, 1, p, MPFR_RNDD);
  mpfr_pow_ui(p, p, k, MPFR_RNDD);
}
// there exists an infection among k fixed points
void fm(mpfr_t p, int k) {
  nm(p, k);
  mpfr_ui_sub(p, 1, p, MPFR_RNDD);
}
// normalizing function for image generation
real normalize(real v) { return fmaxl(logl(v), -200); }

int main(int argc, char **argv) {
  clock_t tic = clock(); // for timing purposes
  int m_min = 4; // we let p run from 2^{-m_min}, 2^{-2}, ..., 2^{-m_max} (inclusive).
  int m_max = 17;

  // loop through several values of p
  for (int m = m_min; m <= m_max; m++) {
    // mpf_t p;
    // mpf_init(p);
    // mpf_set_d(p, powl(2, -m));
    real p;
    p = powl(2, -m);
    int a_max = (int) LOG_MULTIPLE * logl(1 / p) / p;

    mpfr_t long_p;
    mpfr_init(long_p);

    tic = clock();
    // we now loop over the diagonals i + j = s. As we already filled M(1, 1), we start with 3.
    //for (int s = 3; s < a_max; s++) {
    //  if (s == a_max - 1) {
    //printf("%Lf, %Lf # m = %d, p = %7.6Lf\n", -logl(p), -p * logl(sum), m, p);
    printf("m = %d, a_max = %d", m, a_max);
    printf("p = %7.6Lf, \n", p);
    printf("f (p, a_max) = %23.16Lf,\n", f(p, a_max));
    mpfr_set_d(long_p, p, MPFR_RNDD);
    fm(long_p, a_max);
    printf("fm(p, a_max) =  ");
    mpfr_out_str(stdout, 10, 40, long_p, MPFR_RNDD);
    printf("\nn (p, a_max) = %23.16Lf,", n(p, a_max));
    mpfr_set_d(long_p, p, MPFR_RNDD);
    nm(long_p, a_max);
    printf("\nnm(p, a_max) = , ");
    mpfr_out_str(stdout, 10, 40, long_p, MPFR_RNDD);
    printf("\n");
    fflush(stdout);
    //  }
    //}
  }
  fprintf(stderr, "Elapsed in c_30: %Lf secs\n\n", (real)(clock() - tic)/CLOCKS_PER_SEC);
  return 0;
}

void c_modified(
  real p,
  int s,
  void * o,
  void * p1,
  void * p2,
  void * p3,
  int a_max)
{
  real (*output)[a_max] = static_cast<real (*)[a_max]>(o);
  real (*past1)[a_max] = static_cast<real (*)[a_max]>(p1);
  real (*past2)[a_max] = static_cast<real (*)[a_max]>(p2);
  real (*past3)[a_max] = static_cast<real (*)[a_max]>(p3);
  // pragma omp parallel for
  for (int a = 1; a < s; a++) {
    int b = s - a;
    output[0][a] = p * (past2[1][a - 1] + past2[5][a - 1] + past2[6][a - 1])
      + f(p, b) * past1[0][a - 1];
    output[1][a] = (1 - p) * f(p, a) * past1[1][a]
      + n(p, b) * output[0][a] + p * past2[2][a - 1];
    output[5][a] = (1 - p) * f(p, a) * past1[5][a] + p * past2[4][a - 1];
    if (a == 1) {
        output[6][a] = (1 - p) * n(p, b) * past1[6][a - 1];
    } else {
      output[6][a] = (1 - p) * n(p, b) * past1[6][a - 1] + p * p * past3[3][a - 2];
    }
    output[2][a] = (1 - p) * f(p, b) * past1[2][a - 1]
      + p * (1 - p) * past2[3][a - 1]
      + (1 - p) * n(p, a) * output[1][a] + (1 - p) * n(p, b) * output[6][a];
    output[4][a] = (1 - p) * f(p, b) * past1[4][a - 1]
      + p * (1 - p) * past2[3][a - 1] + (1 - p) * n(p, a) * output[5][a];
    output[3][a] = (1 - p) * (1 - p) * f(p, a) * past1[3][a]
      + (1 - p) * n(p, b) * output[2][a] + (1 - p) * n(p, b) * output[4][a];
  }
}
