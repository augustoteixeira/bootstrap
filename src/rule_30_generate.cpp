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


int main(int argc, char **argv) {
  Func hal_30("hal_30");

  Var x("x"), xi("xi");

  //Param<uint64_t> SIZE;

  ImageParam input(type_of<float>(), 1);

  hal_30(x) = cast<float>(
                          select((input(x - 1) > 0) != ((input(x) > 0) | input(x + 1) > 0), 1, 0)
  );

  // schedule
  //hal_30.vectorize(x, 8);
  Var x_micro, x_index, x_mezo, x_vec;
  hal_30.split(x, x_vec, x_micro, 8)
    .vectorize(x_micro);
  //  .split(x_vec, x_index, x_mezo, 1000)
  //  .parallel(x_index);

  hal_30.compile_to_static_library("rule_30_halide", {input}, "hal_30");

  printf("Hal 30 compiled.\n");

  return 0;
}
