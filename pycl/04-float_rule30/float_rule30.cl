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
  int gid = get_global_id(0);
  int left = input[(gid - 1 + width) % width];
  int middle = input[gid];
  int right = input[(gid + 1) % width];

  output[gid] = apply_rule30(left, middle, right);
}
