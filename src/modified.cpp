#include <stdio.h>
#include <time.h>
#include <math.h>

//#include "omp.h"

typedef long double real;

const real LOG_MULTIPLE = 1.5;
// this updates a diagonal (s = a + b) with a given past. To be defined below.
void c_modified(real p, int s, void*, void*, void*, void*, int);
bool IsPowerOfTwo(ulong x) { return (x != 0) && ((x & (x - 1)) == 0); }
// there exists an infection among k fixed points
real f(real p, int k) { return 1 - powl(1 - p, k); }
// there is no infection in these k points
real n(real p, int k) { return powl(1 - p, k); }
// normalizing function for image generation
real normalize(real v) { return fmaxl(logl(v), -200); }

int main(int argc, char **argv) {
  {
    clock_t tic = clock(); // for timing purposes
    int m_min = 4; // we let p run from 2^{-m_min}, 2^{-2}, ..., 2^{-m_max} (inclusive).
    int m_max = 18;
    int m_for_image = 4; // this exponent will be used to build the table and the png image
    if (m_max < m_min) { return 1; };
    //if (m_for_image < m_min) { return 1; };
    //if (m_for_image > m_max) { return 1; };

    // on my machine, reals are 16 bytes long, which is twice as much as double

    // image inicialization
    real p_for_image = powl(2.0, -m_for_image);
    int a_max_for_image = (int) LOG_MULTIPLE * logl(1 / p_for_image) / p_for_image;
    real (*image)[a_max_for_image] = (real (*)[a_max_for_image])
      malloc(sizeof(real[a_max_for_image][a_max_for_image]));
    for (int a = 0; a < a_max_for_image; a++) {
      for (int b = 0; b < a_max_for_image; b++) {
        image[a][b] = 0;
      }
    }

    // table dimensions
    int tab_h = 5;
    int tab_w = 6;
    real (*table)[tab_h][tab_w] = (real (*)[tab_h][tab_w])
      malloc(sizeof(real[7][tab_h][tab_w]));

    // loop through several values of p
    for (int m = m_min; m <= m_max; m++) {
      real p = powl(2, -m);
      int a_max = (int) LOG_MULTIPLE * logl(1 / p) / p;

      // N0, N1, N2, N3 will rotate to progress on the calculation
      // one will be the current one, being updated and the others will be the
      // three past layers.
      // Note that we use 7 * a_max, one for each of the 7 states
      real (*N0)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
      real (*N1)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
      real (*N2)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
      real (*N3)[a_max] = (real (*)[a_max]) malloc(sizeof(real[7][a_max]));
      // these pointers will be set accordingly. To fill M(i,j), we use N(i+j mod 4)
      real (*current)[a_max];
      real (*past1)[a_max];
      real (*past2)[a_max];
      real (*past3)[a_max];
      // initialize everything with zeros first
      for (int k = 0; k < 7 ; k++) {
        for (int s = 0; s < a_max; s++) {
          N0[k][s] = 0; N1[k][s] = 0; N2[k][s] = 0; N3[k][s] = 0;
        }
      }
      // update the value of the 1 x 1 boxes for the states that are reachable:
      // note that we use N2 to hold the value of the M(1,1), because 1 + 1 mod 4 = 2.
      N2[0][1] = table[0][1][1] = p * n(p, 0);
      N2[1][1] = table[1][1][1] = p * n(p, 1);
      N2[2][1] = table[2][1][1] = p * n(p, 3);
      N2[3][1] = table[3][1][1] = p * n(p, 5);
      image[1][1] = p;

      tic = clock();
      // we now loop over the diagonals i + j = s. As we already filled M(1, 1), we start with 3.
      for (int s = 3; s < a_max; s++) {
        // assign the pointers, depending on the role they play in the rotation
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

        // fill the current array with an auxiliary function (implemented below)
        c_modified(p, s, current, past1, past2, past3, a_max);

        // if the m value is the one that we want the table and image, fill up their data
        if (m == m_for_image) {
          // fill table
          for (int k = 0; k < 7; k++) {
            for (int a = 0; a < tab_w; a++) {
              if (s - a <= tab_h) {
                table[k][a][s - a] = current[k][a];
              }
            }
          }
          // fill image
          for (int a = 1; a < s; a++) {
            image[s-a][a] = current[0][a];
          }
        }
        // if we are at the last diagonal, calculate max and sum and print
        if (s == a_max - 1) {
          real sum = 0;
          real max = 0;
          for (int l = 0; l < a_max; l++) {
            if (max < current[0][l]) { max = current[0][l]; }
            sum += current[0][l];
          }
          printf("%Lf, %Lf # m = %d\n", -logl(p), -p * logl(sum), m);
          //printf("m = %d, N = %d, logl(sum{M(x, N-x)}) = %Lf, -logl(middle) = %Lf\n",
          //       m, a_max, logl(sum), -logl(current[s/2]));
          fflush(stdout);
        }
      }
    }
    fprintf(stderr, "Elapsed in c_30: %Lf secs\n\n", (real)(clock() - tic)/CLOCKS_PER_SEC);

    // print tables
    printf("table dimensions = %d by %d\n", tab_h, tab_w);
    for (int k = 0; k < 7; k++) {
      printf("table %d:\n", k);
      for (int b = tab_w - 1; b >= 0; b--) {
        for (int a = 0; a < tab_h; a++) {
          printf("M[%d, %d] = ", a, b);
          real value =
            normalize(table[k][a][b]);
          if (value == -200) {
            printf("         ");
          } else {
            printf("%7.2Lf, ", value);
          }
        }
        printf("\n");
      }
    }
    printf("\n");

    // write image
    printf("p_for_image = %Lf\n", p_for_image);
    printf("log(p) = %Lf\n", logl(p_for_image));
    printf("a_max_for_image = %d\n", a_max_for_image);
    printf("image size = %d\n", a_max_for_image * a_max_for_image);
    // normalize image
    for (int a = 0; a < a_max_for_image; a++) {
      for (int b = 0; b < a_max_for_image; b++) {
        image[a][b] = normalize(image[a][b]);
      }
    }
    // scale it to stay between zero and 255
    real max_image = -10000000000;
    real min_image = 1000000000;
    for (int a = 0; a < a_max_for_image/2; a++) {
      for (int b = 0; b < a_max_for_image/2; b++) {
        if (image[a][b] > max_image) {
          max_image = image[a][b];
        }
        if (image[a][b] < min_image) {
          min_image = image[a][b];
        }
      }
    }
    for (int a = 0; a < a_max_for_image/2; a++) {
      for (int b = 0; b < a_max_for_image/2; b++) {
        image[a][b] = 255 * ((image[a][b] - min_image) / (max_image - min_image));
      }
    }
    // write to file
    FILE *imageFile;
    int x,y,pixel,height=a_max_for_image/2,width=a_max_for_image/2;
    imageFile=fopen("image.pgm","wb");
    if(imageFile==NULL){
      perror("ERROR: Cannot open output file");
      exit(EXIT_FAILURE);
    }
    fprintf(imageFile,"P5\n");           // P5 filetype
    fprintf(imageFile,"%d %d\n",width,height);   // dimensions
    fprintf(imageFile,"255\n");          // Max pixel
    /* Now write a greyscale ramp */
    for(x=0;x<height;x++){
      for(y=0;y<width;y++){
        pixel = (int) image[height - x - 1][y];
        fputc(pixel,imageFile);
      }
    }
    fclose(imageFile);
  }
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
