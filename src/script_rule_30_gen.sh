set -e

echo "Compiling generator..."
g++ rule_30_generate.cpp -g -std=c++17 -I ~/Downloads/Halide-16.0.0-x86-64-linux-1e963ff817ef0968cc25d811a25a7350c8953ee6/Halide-16.0.0-x86-64-linux/include -L ~/Downloads/Halide-16.0.0-x86-64-linux-1e963ff817ef0968cc25d811a25a7350c8953ee6/Halide-16.0.0-x86-64-linux/lib -lHalide -lpthread -ldl -o rule_30_generate

echo "Running generator..."
LD_LIBRARY_PATH=~/Downloads/Halide-16.0.0-x86-64-linux-1e963ff817ef0968cc25d811a25a7350c8953ee6/Halide-16.0.0-x86-64-linux/lib/ ./rule_30_generate

echo "Compiling binary..."
g++ rule_30.cpp rule_30_halide.a -std=c++17 -I ~/Downloads/Halide-16.0.0-x86-64-linux-1e963ff817ef0968cc25d811a25a7350c8953ee6/Halide-16.0.0-x86-64-linux/include -I ~/Downloads/Halide-16.0.0-x86-64-linux-1e963ff817ef0968cc25d811a25a7350c8953ee6/Halide-16.0.0-x86-64-linux/share/Halide/tools/ -L ~/Downloads/Halide-16.0.0-x86-64-linux-1e963ff817ef0968cc25d811a25a7350c8953ee6/Halide-16.0.0-x86-64-linux/lib -lHalide -I /usr/include/libpng16 -lpng16 -ljpeg -lpthread -ldl -o rule_30

echo "Running binary..."
LD_LIBRARY_PATH=~/Downloads/Halide-16.0.0-x86-64-linux-1e963ff817ef0968cc25d811a25a7350c8953ee6/Halide-16.0.0-x86-64-linux/lib/ ./rule_30
