// Rule 30 logic embedded
int apply_rule30(int a, int b, int c) {
    if (a != b) {
        return 1;
    } else if (b != c) {
        return 1;
    } else {
        return 0;
    }
}

__kernel void rule30(__global int* input, __global int* output, const int width) {
    int gid = get_global_id(0);
    int left = input[(gid - 1 + width) % width];
    int middle = input[gid];
    int right = input[(gid + 1) % width];

    output[gid] = apply_rule30(left, middle, right);
}
