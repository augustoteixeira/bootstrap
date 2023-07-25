// Halide rule 30

// On linux, you can compile and run it like so:
// g++ lesson_07*.cpp -g -std=c++17 -I <path/to/Halide.h> -I <path/to/tools/halide_image_io.h> -L <path/to/libHalide.so> -lHalide `libpng-config --cflags --ldflags` -ljpeg -lpthread -ldl -o lesson_07
// LD_LIBRARY_PATH=<path/to/libHalide.so> ./lesson_07

// On os x:
// g++ lesson_07*.cpp -g -std=c++17 -I <path/to/Halide.h> -I <path/to/tools/halide_image_io.h> -L <path/to/libHalide.so> -lHalide `libpng-config --cflags --ldflags` -ljpeg -o lesson_07
// DYLD_LIBRARY_PATH=<path/to/libHalide.dylib> ./lesson_07

#include "Halide.h"
#include <stdio.h>

using namespace Halide;

// Support code for loading pngs.
#include "halide_image_io.h"
#include "rule_30_halide.h"

using namespace Halide::Tools;
// using namespace Halide;

const int SIZE = 40;
const int TIME = 2;

void c_30(float input[], float output[]);

int main(int argc, char **argv) {
  // First we'll declare some Vars to use below.
  // Now we'll express a multi-stage pipeline that blurs an image
  // first horizontally, and then vertically.
  {
    Var x("x"), t("t");
    float u[TIME][SIZE];
    float v[TIME][SIZE];
    for (int j = 0; j < TIME; j++) {
      for (int i = 0; i < SIZE; i++) {
        u[j][i] = 0.0;
        v[j][i] = 0.0;
      }
    }
    u[0][SIZE/2] = 1.0;
    v[0][SIZE/2] = 1.0;

    for (int j = 1; j < TIME; j++) {
      Halide::Runtime::Buffer input(v[j - 1]), output(v[j]);
      int error = hal_30(input, SIZE, output);
      if (error) {
        printf("Halide returned an error: %d\n", error);
      }
      c_30(u[j-1], u[j]);
    }

    Halide::Buffer<float> buf(u);
    Func buf_u8("output");
    buf_u8(x, t) = cast<uint8_t>(200 * (1 - buf(x, t)));
    Halide::Buffer<uint8_t> output = buf_u8.realize({40, 20});
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

// void hal_30(float input[], float output[]) {
//   Var x("x");
//   // Func u("u"), v("v");
//   Func v("u");
//   // u(x) = 0;
//   // u(x) = 0;
//   // u(21) = 1;
//   // //v(x, 0) = u(x, 0);
//   // //v.trace_stores();
//   // //for (int i = 1; i < 20; i++) {
//   Halide::Buffer<float> input_buf(input);

//   v(x) = select((input_buf(x) > 0) | (input_buf(x) > 0), 1, 0);

//   Halide::Buffer<float> output_buf = v.realize({SIZE, TIME});

//   output = *output_buf;
//   //Halide::Buffer<float> output_buf(input);
//   // u(x) = v(x, i);
//   // // }
//   // Func fa[20];
//   // fa[0](x) = u(x);
//   // for (int i = 1; i < 20; i++) {
//   //   fa[i](x) = select((fa[i - 1](x + 1)) != (fa[i - 1](x) > 0) | (fa[i - 1](x - 1) > 0), 1, 0);
//   //   //h(x, r) = Tuple(u(x, r - 1), u(x, r - 1));
//   //   //v(x, i) = select((v(x, i - 1) > 0) | (v(x, i - 1) > 0), 1, 0);
//   //   //u(x, r) = h(x, r)[0] + h(x, r)[1]; //select((u(x, i - 1) > 0) | (u(x - 1, i - 1) > 0), 1, 0);
//   // }



//   // Func config("config");
//   // config(x, t) = select(fa[19](x) == 0, 255, 0);
//   // Func config_u8("config_u8");
//   // config_u8(x, t) = cast<uint8_t>(config(x, t));

//   // //Buffer<uint8_t> rect(40, 20);
//   // //rect.set_min(-20, 0);

//   // Halide::Buffer<uint8_t> output = config_u8.realize({40, 20});
// }
