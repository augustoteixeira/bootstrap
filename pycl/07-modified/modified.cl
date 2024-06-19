

double add(double a, double b);
double sub(double a, double b);
double f(double log_p, double n);
double n(double log_p, double n);

double add(double a, double b) {
    if (isinf(a) && isinf(b)) {
        return -INFINITY;
    }
    double maxim = fmax(a, b);
    double minim = fmin(a, b);
    return maxim + log(1.0 + exp(minim - maxim));
}

double sub(double a, double b) {
    if (isinf(a) && isinf(b)) {
        return -INFINITY;
    }
    double maxim = fmax(a, b);
    double minim = fmin(a, b);
    return maxim + log(1.0 - exp(minim - maxim));
}

double f(double log_p, double n) {
    return sub(0.0, sub(0.0, log_p) * n);
}

double n(double log_p, double n) {
    return sub(0.0, log_p) * n;
}

__kernel void modified
  (
    __global double* o,
    __global double* p1,
    __global double* p2,
    __global double* p3,
    const double p,
    const int a,
    const int s
  )
{
    int i = get_global_id(1);
    int j = get_global_id(1);

    double zero = -INFINITY;
    double one = 0.0;
    double q = sub(one, p);

    if (i > 0 && i < s)
    //for (int k = 1; k < s; k++)
    {
        int b = s - i;
        o[0 * a + i] = add(p + add(add(p2[1 * a + i - 1], p2[5 * a + i - 1]), p2[6 * a + i - 1]),
                      f(p, b) + p1[0 * a + i - 1]);
        o[1 * a + i] = add(add(q + f(p, i) + p1[1 * a + i], n(p, b) + o[0 * a + i]),
                      p + p2[2 * a + i - 1]);
        o[5 * a + i] = add(q + f(p, i) + p1[5 * a + i], p + p2[4 * a + i - 1]);
        if (i < 3) {
            o[6 * a + i] = zero;
        } else {
            o[6 * a + i] = add(q + f(p, b) + p1[6 * a + i - 1], p + p + p3[3 * a + i - 2]);
        }
        o[2 * a + i] = add(add(add(q + f(p, b) + p1[2 * a + i - 1], p + q + p2[3 * a + i - 1]),
                      q + n(p, i) + o[1 * a + i]), q + n(p, b) + o[6 * a + i]);
        o[4 * a + i] = add(add(q + f(p, b) + p1[4 * a + i - 1], p + q + p2[3 * a + i - 1]),
                      q + n(p, i) + o[5 * a + i]);
        o[3 * a + i] = add(add(q + q + f(p, i) + p1[3 * a + i], q + n(p, b) + o[2 * a + i]),
                      q + n(p, b) + o[4 * a + i]);
    }
}