import taichi as ti
import numpy as np
import taichi.math as tm
import math

@ti.func
def sub(a, b):
    ans = 0.0
    if tm.isinf(a) and tm.isinf(b):
        ans = -math.inf
    else:
        maxim = tm.max(a, b)
        minim = tm.min(a, b)
        ans = maxim + tm.log(1.0 - tm.exp(minim - maxim))
    return ans

def add_py(a, b):
    if math.isinf(a) and math.isinf(b):
        return -math.inf
    maxim = max(a, b)
    minim = min(a, b)
    return maxim + math.log(1.0 + math.exp(minim - maxim))

@ti.func
def add(a, b):
    ans = 0.0
    if tm.isinf(a) and tm.isinf(b):
        ans = -math.inf
    else:
        maxim = tm.max(a, b)
        minim = tm.min(a, b)
        ans = maxim + tm.log(1.0 + tm.exp(minim - maxim))
    #if tm.isnan(ans):
    #    print(f"Error with {a} + {b}")
    return ans

@ti.func
def n(log_p, n):
    return (sub(0.0, log_p)) * n

@ti.func
def f(log_p, n):
    return sub(0.0, sub(0.0, log_p) * n)

m_min = 2
m_max = 2

m_table = 2
table_size = 5

log_multiple = 1.5

#ti.init(arch=ti.gpu, default_fp=ti.f64)
ti.init(arch=ti.cpu, default_fp=ti.f64)

@ti.kernel
def init(log_p: float, n2: ti.template()):
    n2[0, 1] = log_p
    print(ti.exp(log_p))
    n2[1, 1] = log_p + n(log_p, 1)
    n2[2, 1] = log_p + n(log_p, 2)
    n2[3, 1] = log_p + n(log_p, 3)

@ti.kernel
def frobose(p: float, s: int,
             o: ti.template(),
             past1: ti.template(),
             past2: ti.template(),
             past3: ti.template(),
             past4: ti.template()):
    zero = -math.inf
    one = 0.0
    four = tm.log(4.0)

    q = sub(one, log_p)
    p2 = p + p
    q2 = q + q
    pq = p + q
    pq2 = p + q + q
    p2q2 = p + p + q + q
    p3q = p + p + p + q
    p4 = p + p + p + p

    for aa in range(1, s):
        b = s - aa
        #print(f"{p}, {s}: past2[1, {aa} - 1]={past2[1, aa - 1]}, past2[5, {aa} - 1]={past2[5, aa - 1]},past2[6, {aa} - 1]={past2[6, aa - 1]},f(p, {b})={f(p, b)},past1[0, {aa} - 1]={past1[0, aa - 1]}")
        o[0, aa] = add(f(p, b) + past1[0, aa - 1],
                       add(f(p, aa) + p + past2[1, aa - 1],
                           add(f(p, aa) + p + past2[5, aa - 1],
                               f(p, b) + p + past2[6, aa - 1])))
        if a >= 3:
            o[0, aa] = add(o[0, aa],
                           add(f(p, b) + p2 + past3[2, aa - 2],
                               add(four + add(p3q, p4) + f(p, aa) + past4[3, aa - 2],
                                   f(p, b) + p2 + past3[4, aa - 2])))
        o[1, aa] = add(n(p, b) + o[0, aa],
                       add(f(p, aa) + q + past1[1, a],
                           add(f(p, b) + pq + past2[2, aa - 1],
                               f(p, aa) + p2q2 + past3[3, aa - 1])))
        o[6, aa] = f(p, b) + q + past1[6, aa - 1]
        if a >= 3:
            o[6, aa] = add(o[6, aa], f(p, aa))
        o[5, aa] = add(f(p, aa) + p2q2 + past3[3, aa - 1],
                       add(f(p, b) + pq + past2[4, aa - 1],
                           f(p, aa) + q + past1[4, a - 1]))
        o[4, aa] = add(f(p, aa) + pq2 + past2[3, aa - 1],
                       add(f(p, b) + q + past1[4, aa - 1],
                           n(p, aa) + o[5, aa]))
        o[2, aa] = add(n(p, aa) + o[1, aa],
                       add(f(p, b) + q + past1[2, aa - 1],
                           add(f(p, aa) + pq2 + past2[3, aa - 1],
                               n(p, b) + o[6, a])))
        o[3, aa] = add(n(p, b) + o[2, aa],
                       add(f(p, aa) + q2 + past1[3, aa],
                           n(p, b) + o[4, aa]))
        #print(f"s = {s}, o[0, {aa}] = {o[0, aa]}")

def fill_diagonal(s, current, table):
    for k in range(0, 7):
        for j in range(1, s):
            if j < table_size:
                if s - j < table_size:
                    table[k][j][s - j] = math.exp(current[k, j])

for m in range(m_min, m_max + 1):
    p_float = 2**(-m)
    a = math.floor(log_multiple * math.log(1.0 / p_float) / p_float)

    #print(f"a = {a}")

    log_p =  tm.log(p_float)

    n0 = ti.field(ti.f64)
    n1 = ti.field(ti.f64)
    n2 = ti.field(ti.f64)
    n3 = ti.field(ti.f64)
    n4 = ti.field(ti.f64)

    table = np.zeros((7, table_size, table_size))

    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n0)
    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n1)
    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n2)
    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n3)
    ti.root.dense(ti.i, 7).dense(ti.j, a).place(n4)

    n0.fill(-math.inf)
    n1.fill(-math.inf)
    n2.fill(-math.inf)
    n3.fill(-math.inf)
    n4.fill(-math.inf)

    init(log_p, n2)

    for s in range(3, a):
        if s % 5 == 0:
            #for i in range(1, s):
            #    print(f"0 s = {s}, s%3 = {s%3}, p1[0, {i}] = {n3[0, i]}")
            frobose(log_p, s, n0, n4, n3, n2, n1)
            if m == m_table:
                fill_diagonal(s, n0, table)
        if s % 5 == 1:
            #for i in range(1, s):
            #    print(f"1 s = {s}, p1[0, {i}] = {n0[0, i]}")
            frobose(log_p, s, n1, n0, n4, n3, n2)
            if m == m_table:
                fill_diagonal(s, n1, table)
        if s % 5 == 2:
            #for i in range(1, s):
            #    print(f"2 s = {s}, p1[0, {i}] = {n1[0, i]}")
            frobose(log_p, s, n2, n1, n0, n4, n3)
            if m == m_table:
                fill_diagonal(s, n2, table)
        if s % 5 == 3:
            #for i in range(1, s):
            #    print(f"3 s = {s}, p1[0, {i}] = {n2[0, i]}")
            frobose(log_p, s, n3, n2, n1, n0, n4)
            if m == m_table:
                fill_diagonal(s, n3, table)
        if s % 5 == 3:
            #for i in range(1, s):
            #    print(f"3 s = {s}, p1[0, {i}] = {n2[0, i]}")
            frobose(log_p, s, n4, n3, n2, n1, n0)
            if m == m_table:
                fill_diagonal(s, n4, table)

    # summing the secondary diagonal
    acc = -math.inf
    for l in range(a):
        # for k in range(7):
            k = 0
            if s % 5 == 0:
                acc = add_py(acc, n0[k, l])
            if s % 5 == 1:
                acc = add_py(acc, n1[k, l])
            if s % 5 == 2:
                acc = add_py(acc, n2[k, l])
            if s % 5 == 3:
                acc = add_py(acc, n3[k, l])
            if s % 5 == 4:
                acc = add_py(acc, n4[k, l])

    print(f"p = {p_float}, size = {a}, -p log(s) = {-p_float * acc}, m = {m}")
    file = open(f'result_{m:02d}.txt', 'w')
    file.write(f"p = {p_float}, size = {a}, -p log(s) = {-p_float * acc}, m = {m}\n")
    file.close()

#print(table)
for k in range(0, 7):
    np.savetxt(f"table_{k}.csv", table[k][:][:], delimiter=",")
