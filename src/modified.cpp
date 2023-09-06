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

const long double LOG_MULTIPLE = 2.0;


void c_modified(long double p, int s, long double output[], long double past1[], long double past2[], long double past3[], int a_max);

bool IsPowerOfTwo(ulong x)
{
  return (x != 0) && ((x & (x - 1)) == 0);
}

long double f(long double p, int k) {
  return 1 - powl(1 - p, k);
}

long double n(long double p, int k) {
  return powl(1 - p, k);
}

long double normalize(long double v) {
  return fmaxl(logl(v), -200);
}

int main(int argc, char **argv) {
  {
    clock_t tic = clock();
    int m_max = 4;
    int m_for_image = 4; // this will be used to build the table and the png image
    if (m_max < m_for_image) { return 1; };

    // on my machine, long doubles are 16 bytes long, which is twice as much as double

    // image inicialization
    long double p_for_image = powl(2.0, -m_for_image);
    int a_max_for_image = (int) LOG_MULTIPLE * logl(1 / p_for_image) / p_for_image;
    long double* image = new long double[a_max_for_image * a_max_for_image];
    for (int j = 0; j < a_max_for_image * a_max_for_image; j++) { image[j] = 0; }

    // table dimensions
    int tab_h = 5;
    int tab_w = 6;
    long double* table = new long double[tab_h * tab_w * 7];

    // loop through several values of p
    for (int m = 1; m <= m_max; m++) {
      long double p = powl(2, -m);
      int a_max = (int) LOG_MULTIPLE * logl(1 / p) / p;

      // N0, N1, N2, N3 will rotate to progress on the calculation
      // one will be the current one, being updated and the others will be the
      // three past layers.
      // Note that we use 7 * a_max, one for each of the 7 states
      long double* N0 = new long double[7 * a_max];
      long double* N1 = new long double[7 * a_max];
      long double* N2 = new long double[7 * a_max];
      long double* N3 = new long double[7 * a_max];
      // these pointers will be set accordingly. To fill M(i,j), we use N(i+j mod 4)
      long double* current;
      long double* past1;
      long double* past2;
      long double* past3;
      // initialize everything with zeros first
      for (int j = 0; j < 7 * a_max; j++) {
        N0[j] = 0; N1[j] = 0; N2[j] = 0; N3[j] = 0;
      }
      // update the value of the 1 x 1 boxes for the states that are reachable:
      N2[0*a_max+1] = image[1*a_max_for_image + 1] = table[0*tab_h*tab_w + tab_h + 1] = p * n(p, 0);
      N2[1*a_max+1] = image[2*a_max_for_image + 1] = table[1*tab_h*tab_w + tab_h + 1] = p * n(p, 1);
      N2[2*a_max+1] = image[3*a_max_for_image + 1] = table[2*tab_h*tab_w + tab_h + 1] = p * n(p, 3);
      N2[3*a_max+1] = image[4*a_max_for_image + 1] = table[3*tab_h*tab_w + tab_h + 1] = p * n(p, 5);
      // note that we use N2 to hold the value of the M(1,1), because 1 + 1 mod 4 = 2.

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
            for (int i = 0; i < tab_w; i++) {
              if (s - i <= tab_h) {
                table[k * tab_w * tab_h + i * tab_h + (s - i)] = current[k * a_max + i];
              }
            }
          }
          // fill image
          for (int j = 1; j < s; j++) {
            image[(s-j) * a_max + j] = current[j];
          }
        }
        // if we are at the last diagonal, calculate max and sum and print
        if (s == a_max - 1) {
          long double sum = 0;
          long double max = 0;
          for (int l = 0; l < a_max; l++) {
            if (max < current[l]) { max = current[l]; }
            sum += current[l];
          }
          printf("%Lf, %Lf\n", -logl(p), -p * logl(sum));
          //printf("m = %d, N = %d, logl(sum{M(x, N-x)}) = %Lf, -logl(middle) = %Lf\n",
          //       m, a_max, logl(sum), -logl(current[s/2]));
          fflush(stdout);
        }
      }

      if (m == m_for_image) {
        // print table
        printf("table size = %d\n", tab_h * tab_w * 7);
        for (int k = 0; k < 7; k++) {
          printf("table %d:\n", k);
          for (int i = tab_w - 1; i > 0; i--) {
            for (int j = 1; j < tab_h; j++) {
              printf("M[%d, %d] = ", j, i);
              long double value =
                normalize(table[k * tab_w * tab_h + j * tab_h + i]);
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
        printf("a_max_for_image = %d\n", a_max_for_image);
        printf("image size = %d\n", a_max_for_image * a_max_for_image);
        for (int j = 0; j < a_max_for_image * a_max_for_image; j++) {
          image[j] = normalize(image[j]);
        }
        // for (int j = 5; j > 0; j--) {
        //   for (int i = 1; i < 6; i++) {
        //     printf("i[%d, %d] = ", i, j);
        //     long double value = image[j * a_max + i];
        //     if (value == -200) {
        //       printf("         ");
        //     } else {
        //       printf("%7.2Lf, ", value);
        //     }
        //   }
        //   printf("\n");
        // }
        printf("p = %Lf\n", p);
        printf("log p = %Lf\n", logl(p));
        printf("N(1) = %Lf\n", n(p, 1));

        long double max_image = -10000000000;
        long double min_image = 1000000000;
        for (int i = 0; i < a_max_for_image/2; i++) {
          for (int j = 0; j < a_max_for_image/2; j++) {
            if (image[i * a_max_for_image + j] > max_image) {
              max_image = image[i * a_max_for_image + j];
            }
            if (image[i * a_max_for_image + j] < min_image) {
              min_image = image[i * a_max_for_image + j];
            }
          }
        }
        printf("max_image = %Lf\n", max_image);
        printf("min_image = %Lf\n", min_image);
        for (int i = 0; i < a_max_for_image/2; i++) {
          for (int j = 0; j < a_max_for_image/2; j++) {
            image[i * a_max_for_image + j] =
              255 * ((image[i * a_max_for_image + j] - min_image) / (max_image - min_image));
          }
        }

        FILE *imageFile;
        int x,y,pixel,height=a_max/2,width=a_max/2;

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
            pixel = (int) image[(height - x - 1) * a_max + y];
            fputc(pixel,imageFile);
          }
        }

        fclose(imageFile);
      }
      delete N0;
      delete N1;
      delete N2;
      delete N3;
    }
    fprintf(stderr, "Elapsed in c_30: %Lf secs\n", (long double)(clock() - tic)/CLOCKS_PER_SEC);
    delete image;
  }
  return 0;
}

void c_modified(
  long double p,
  int s,
  long double output[],
  long double past1[],
  long double past2[],
  long double past3[],
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
