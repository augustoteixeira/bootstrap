bool distinct(double a, double b)
{
  return a > b + 0.000001
      || b > a + 0.000001;
}

// Rule 30 logic embedded
double apply_rule30(double a, double b, double c) {
  if (distinct(a, b)) {
    return 1.0;
  } else if (distinct(b, c)) {
    return 1.0;
  } else {
    return 0.0;
  }
}

__kernel void rule30(__global double* input, __global double* output, const int width) {
  int y = get_global_id(0);
  int x = get_global_id(1);

  double left = input[(x - 1) % width + y * width];
  double middle = input[x + y * width];
  double right = input[(x + 1) % width + y * width];

  output[x + y * width] = apply_rule30(left, middle, right);
}
