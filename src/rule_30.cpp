// Halide rule 30

// On linux, you can compile and run it like so:
// g++ lesson_07*.cpp -g -std=c++17 -I <path/to/Halide.h> -I <path/to/tools/halide_image_io.h> -L <path/to/libHalide.so> -lHalide `libpng-config --cflags --ldflags` -ljpeg -lpthread -ldl -o lesson_07
// LD_LIBRARY_PATH=<path/to/libHalide.so> ./lesson_07

// On os x:
// g++ lesson_07*.cpp -g -std=c++17 -I <path/to/Halide.h> -I <path/to/tools/halide_image_io.h> -L <path/to/libHalide.so> -lHalide `libpng-config --cflags --ldflags` -ljpeg -o lesson_07
// DYLD_LIBRARY_PATH=<path/to/libHalide.dylib> ./lesson_07

#include "Halide.h"
#include <stdio.h>
#include <time.h>

using namespace Halide;

// Support code for loading pngs.
#include "halide_image_io.h"
#include "rule_30_halide.h"

using namespace Halide::Tools;
// using namespace Halide;

const int SIZE = 10000;
const int TIME = 5000;

void c_30(float input[], float output[]);

int main(int argc, char **argv) {
  {
    Var x("x"), t("t");
    float* u = new float[TIME * SIZE];
    float* v = new float[TIME * SIZE];
    float* w = new float[TIME * SIZE];
    float* ping = new float[SIZE];
    float* pong = new float[SIZE];
    for (int s = 0; s < TIME; s++) {
      for (int y = 0; y < SIZE; y++) {
        u[s * SIZE + y] = 0.0;
        v[s * SIZE + y] = 0.0;
        w[s * SIZE + y] = 0.0;
      }
    }
    for (int y = 0; y < SIZE; y++) {
      ping[y] = 0.0;
      pong[y] = 0.0;
    }
    u[SIZE/2] = 1.0;
    v[SIZE/2] = 1.0;
    w[SIZE/2] = 1.0;
    ping[SIZE/2] = 1.0;

    clock_t tic = clock();
    for (int s = 1; s < TIME; s++) {
      Halide::Runtime::Buffer input(v + (s - 1) * SIZE, SIZE);
      Halide::Runtime::Buffer<float> output(v + s * SIZE + 1, SIZE - 2);
      output.set_min(1);
      int error = hal_30(input, output);
      if (error) {
        printf("Halide returned an error: %d\n", error);
      }
    }
    printf("Elapsed in hal_30: %f secs\n", (double)(clock() - tic)/CLOCKS_PER_SEC);

    tic = clock();
    for (int s = 1; s < TIME; s++) {
      float *input_ptr, *output_ptr;
      if (s % 2 == 1) {
        input_ptr = ping;
        output_ptr = pong;
      } else {
        input_ptr = pong;
        output_ptr = ping;
      }
      Halide::Runtime::Buffer input(input_ptr, SIZE);
      Halide::Runtime::Buffer<float> output(output_ptr + 1, SIZE - 2);
      output.set_min(1);
      int error = hal_30(input, output);
      output_ptr[0] = 0.0;
      output_ptr[SIZE - 1] = 0.0;
      //for (int y = 0; y < SIZE; y++) {
      //  w[s * SIZE + y] = output_ptr[y];
      //}
      if (error) {
        printf("Halide returned an error: %d\n", error);
      }
    }
    printf("Elapsed in hal_30 2: %f secs\n", (double)(clock() - tic)/CLOCKS_PER_SEC);

    tic = clock();
    for (int s = 1; s < TIME; s++) {
      c_30(u + (s - 1) * SIZE, u + s * SIZE);
    }
    printf("Elapsed in c_30: %f secs\n", (double)(clock() - tic)/CLOCKS_PER_SEC);

    for (int s = 0; s < TIME; s++) {
      for (int y = 0; y < SIZE; y++) {
        if (u[s * SIZE + y] != v[s * SIZE + y]) {
          printf("Error at (%d, %d), u is %f, v is %f\n",
                 s, y, u[s * SIZE + y], v[s * SIZE + y]);
        }
      }
    }
    int final_time = TIME - 1;
    float *final_ptr;
    if (final_time % 2 == 0) {
      final_ptr = ping;
    } else {
      final_ptr = pong;
    }
    for (int y = 0; y < SIZE; y++) {
      if (v[final_time * SIZE + y] != final_ptr[y]) {
        printf("Error at (%d, %d), u is %f, ping-pong is %f\n",
               final_time, y, v[final_time * SIZE + y], final_ptr[y]);
      }
    }

    Halide::Buffer<float> buf(v, SIZE, TIME);
    Func buf_u8("output");
    buf_u8(x, t) = cast<uint8_t>(200 * (1 - buf(x, t)));
    Halide::Buffer<uint8_t> output = buf_u8.realize({SIZE, TIME});
    save_image(output, "rule_30.png");
  }
  return 0;
}

void c_30(float input[], float output[]) {
  for (int i = 1; i < SIZE - 1; i++) {
    output[i] = (input[i - 1] > 0 ? true : false)
      != ((input[i] > 0 ? true : false) | (input[i + 1] > 0 ? true : false));
  }
}
