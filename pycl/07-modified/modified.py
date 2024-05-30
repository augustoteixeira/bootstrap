import pyopencl as cl
import numpy as np
import math
import time

def add_py(a, b):
    if math.isinf(a) and math.isinf(b):
        return -math.inf
    maxim = max(a, b)
    minim = min(a, b)
    return maxim + math.log(1.0 + math.exp(minim - maxim))

m_min = 2
m_max = 5

m_table = 2
table_size = 5

log_multiple = 2.5

restrict_support = False

def fill_diagonal(s, current, table):
    for k in range(0, 7):
        for j in range(1, s):
            if j < table_size:
                if s - j < table_size:
                    table[k][j][s - j] = math.exp(current[k, j])

def clip(x, lo, hi):
    return min(max(x, lo), hi)

def update_supp(old_lo, old_hi, s, current):
    if not restrict_support:
        return 1, s
    res_lo = 1
    res_hi = s
    for lo in range(old_lo - 20, old_lo + 20):
        if lo >=0:
            if lo < s:
                if not math.isinf(current[0, lo]):
                    res_lo = lo
                    break
    for hi in range(old_hi + 20, old_lo - 20, -1):
        if hi >=0:
            if hi < s:
                if not math.isinf(current[0, hi]):
                    res_hi = hi
                    break
    return clip(res_lo - 20, 1, s), clip(res_hi + 20, 1, s)


for m in range(m_min, m_max + 1):
    # Read kernel code from external file
    with open("modified.cl", "r") as file:
        kernel_code = file.read()

    context = cl.create_some_context()
    queue = cl.CommandQueue(context)
    # Load and compile the kernel
    program = cl.Program(context, kernel_code).build()

    start_time = time.time()
    p_float = 2**(-m)
    a = math.floor(log_multiple * math.log(1.0 / p_float) / p_float)

    #print(f"a = {a}")

    log_p =  tm.log(p_float)


    # Initialize input buffer
    n0 = np.zeros(a, dtype=np.double)
    n0_buffer = cl.Buffer(context, cl.mem_flags.READ_WRITE | cl.mem_flags.COPY_HOST_PTR, hostbuf=n0)
    n1 = np.zeros(a, dtype=np.double)
    n1_buffer = cl.Buffer(context, cl.mem_flags.READ_WRITE | cl.mem_flags.COPY_HOST_PTR, hostbuf=n1)
    n2 = np.zeros(a, dtype=np.double)
    n2_buffer = cl.Buffer(context, cl.mem_flags.READ_WRITE | cl.mem_flags.COPY_HOST_PTR, hostbuf=n2)
    n3 = np.zeros(a, dtype=np.double)
    n3_buffer = cl.Buffer(context, cl.mem_flags.READ_WRITE | cl.mem_flags.COPY_HOST_PTR, hostbuf=n3)

    table = np.zeros((7, table_size, table_size))

    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n0)
    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n1)
    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n2)
    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n3)

    n0.fill(-math.inf)
    n1.fill(-math.inf)
    n2.fill(-math.inf)
    n3.fill(-math.inf)

    init(log_p, n2)
    if m == m_table:
        fill_diagonal(2, n2, table)

    supp_lo = 1
    supp_hi = 3
    for s in range(3, a):
        if s % 4 == 0:
            supp_lo, supp_hi = update_supp(supp_lo, supp_hi, s, n0)
            modified(log_p, s, n0, n3, n2, n1, supp_lo, supp_hi)
            if m == m_table:
                fill_diagonal(s, n0, table)
        if s % 4 == 1:
            supp_lo, supp_hi = update_supp(supp_lo, supp_hi, s, n1)
            modified(log_p, s, n1, n0, n3, n2, supp_lo, supp_hi)
            if m == m_table:
                fill_diagonal(s, n1, table)
        if s % 4 == 2:
            supp_lo, supp_hi = update_supp(supp_lo, supp_hi, s, n2)
            modified(log_p, s, n2, n1, n0, n3, supp_lo, supp_hi)
            if m == m_table:
                fill_diagonal(s, n2, table)
        if s % 4 == 3:
            supp_lo, supp_hi = update_supp(supp_lo, supp_hi, s, n3)
            modified(log_p, s, n3, n2, n1, n0, supp_lo, supp_hi)
            if m == m_table:
                fill_diagonal(s, n3, table)

    acc = -math.inf
    for l in range(a):
        # for k in range(7):
            k = 0
            if s % 4 == 0:
                acc = add_py(acc, n0[k, l])
            if s % 4 == 1:
                acc = add_py(acc, n1[k, l])
            if s % 4 == 2:
                acc = add_py(acc, n2[k, l])
            if s % 4 == 3:
                acc = add_py(acc, n3[k, l])

    print(f"p = {p_float}, size = {a}, -p log(s) = {-p_float * acc}, m = {m}, "
          + f"t = {time.time() - start_time}")
    file = open(f'result_{m:02d}.txt', 'w')
    file.write(f"p = {p_float}, size = {a}, -p log(s) = {-p_float * acc}, m = {m}\n")
    file.close()

#print(table)
for k in range(0, 7):
    np.savetxt(f"table_{k}.csv", table[k][:][:], delimiter=",")



# Kernel code for applying Rule 30

def main():

    # Initialize OpenCL context and command queue
    # use the code below to automatically pick a device
    #platform = cl.get_platforms()[0]
    #device = platform.get_devices()[0]
    #context = cl.Context([device])


    # Parameters
    width = 100  # Width of the automaton
    iterations = 50  # Number of iterations to simulate


    # Create output buffer
    output_state = np.zeros(width, dtype=np.double)
    output_buffer = cl.Buffer(context, cl.mem_flags.READ_WRITE, size=output_state.nbytes)

    # Simulation loop
    for i in range(iterations):
        # Execute the kernel
        program.rule30(queue, (width,), None, input_buffer, output_buffer, np.int32(width))

        # Swap input and output buffers for next iteration
        input_buffer, output_buffer = output_buffer, input_buffer

    # Read the final state from the device to the host
    cl.enqueue_copy(queue, output_state, input_buffer).wait()

    # Print the final state
    print(''.join(['*' if cell > 0.5 else ' ' for cell in output_state]))

if __name__ == "__main__":
    main()
