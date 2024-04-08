use csv::WriterBuilder;
use grid::*;
use ndarray::{Array2, Array3, Axis};
use ndarray_csv::Array2Writer;
use std::fs::File;
use std::time::Instant;

mod alg;
mod floats;
mod log;

////////////// if using floating points directly:
// use alg::*;
// type Real = Alg<f64>;

////////////// if storing logs instead:
use log::*;
type Real = Log<f64>;

fn n(p: Real, n: i32) -> Real {
    (Real::new(1.0) - p).powi(n)
}

fn f(p: Real, n: i32) -> Real {
    Real::new(1.0) - (Real::new(1.0) - p).powi(n)
}

const TABLESIZE: usize = 5;

// only used when building a table, should never be time sensitive
fn fill_table_diag(s: usize, current: &Grid<Real>, table: &mut Array3<f64>) {
    for k in 0..7 {
        for j in 1..s {
            if j < TABLESIZE {
                if s - j < TABLESIZE {
                    table[[k, j, s - j]] = current[(k, j)].ln_to_float().exp()
                }
            }
        }
    }
}

fn main() {
    let log_multiple: f64 = 2.0;
    let zero = Real::new(0.0);
    let two = Real::new(2.0);
    let m_min = 2;
    let m_max = 6;
    let m_table = 2; // used to build the tables
    let store_table = true;

    let start = Instant::now(); // for timing the execution
    for m in m_min..=m_max {
        let p: Real = two.powi(-m);

        let a: usize = (log_multiple * (1.0 / p.to_float()).ln() / p.to_float())
            .floor() as usize;

        let mut n0: Grid<Real> = Grid::new(7, a); // the rotating buffers
        let mut n1: Grid<Real> = Grid::new(7, a);
        let mut n2: Grid<Real> = Grid::new(7, a);
        let mut n3: Grid<Real> = Grid::new(7, a);
        let mut n4: Grid<Real> = Grid::new(7, a);

        let mut current: &mut Grid<Real>; // pointers for the rotating buffers
        let mut past1: &mut Grid<Real>;
        let mut past2: &mut Grid<Real>;
        let mut past3: &mut Grid<Real>;
        let mut past4: &mut Grid<Real>;

        // initialization
        for k in 0..7 {
            for s in 0..a {
                n0[(k, s)] = zero;
                n1[(k, s)] = zero;
                n2[(k, s)] = zero;
                n3[(k, s)] = zero;
                n4[(k, s)] = zero;
            }
        }
        n2[(0, 1)] = p * n(p, 0);
        n2[(1, 1)] = p * n(p, 1);
        n2[(2, 1)] = p * n(p, 2);
        n2[(3, 1)] = p * n(p, 3);

        let mut table = Array3::<f64>::zeros((7, TABLESIZE, TABLESIZE));
        current = &mut n2;
        fill_table_diag(2, current, &mut table);

        for s in 3..a {
            // assign the pointers
            match s % 5 {
                0 => {
                    current = &mut n0;
                    past1 = &mut n4;
                    past2 = &mut n3;
                    past3 = &mut n2;
                    past4 = &mut n1;
                }
                1 => {
                    current = &mut n1;
                    past1 = &mut n0;
                    past2 = &mut n4;
                    past3 = &mut n3;
                    past4 = &mut n2;
                }
                2 => {
                    current = &mut n2;
                    past1 = &mut n1;
                    past2 = &mut n0;
                    past3 = &mut n4;
                    past4 = &mut n3;
                }
                3 => {
                    current = &mut n3;
                    past1 = &mut n2;
                    past2 = &mut n1;
                    past3 = &mut n0;
                    past4 = &mut n4;
                }
                4 => {
                    current = &mut n4;
                    past1 = &mut n3;
                    past2 = &mut n2;
                    past3 = &mut n1;
                    past4 = &mut n0;
                }
                _ => unreachable!(),
            }
            // update the current diagonal
            frobose(
                p,
                s,
                &mut current,
                &mut past1,
                &mut past2,
                &mut past3,
                &mut past4,
            );

            // update table
            if (m == m_table) && store_table {
                fill_table_diag(s, current, &mut table);
                if s == a - 1 {
                    for k in 0..7 {
                        let array: &Array2<f64> =
                            &(table.index_axis(Axis(0), k).clone().to_owned());
                        let file =
                            File::create(format!("table_{}.csv", k)).unwrap();
                        let mut writer = WriterBuilder::new()
                            .has_headers(false)
                            .from_writer(file);
                        writer.serialize_array2(array).unwrap();
                    }
                }
            }
        }

        // print final result for this m
        match (a - 1) % 5 {
            0 => {
                current = &mut n0;
            }
            1 => {
                current = &mut n1;
            }
            2 => {
                current = &mut n2;
            }
            3 => {
                current = &mut n3;
            }
            4 => {
                current = &mut n4;
            }
            _ => unreachable!(),
        }

        let mut sum: Real = zero;
        for l in 0..a {
            sum = sum + current[(0, l)];
        }
        println!(
            "p = {}, size = {}, -p * log(diag sum) = {}, m = {}, t = {:?}",
            p,
            a,
            -(p.to_float() * sum.ln_to_float()),
            m,
            start.elapsed()
        );
    }
}

fn frobose(
    p: Real,
    s: usize,
    o: &mut Grid<Real>,
    past1: &mut Grid<Real>,
    past2: &mut Grid<Real>,
    past3: &mut Grid<Real>,
    past4: &mut Grid<Real>,
) {
    let one = Real::new(1.0);
    let four = Real::new(4.0);

    let q = one - p;
    let p2 = p * p;
    let q2 = q * q;
    let pq = p * q;
    let pq2 = p * q * q;
    let p2q2 = p * p * q * q;
    let p3q = p * p * p * q;
    let p4 = p * p * p * p;

    for a in 1..s {
        let b: i32 = (s - a) as i32;
        let aa = a as i32;
        o[(0, a)] = f(p, b) * past1[(0, a - 1)] //                0 -> 0
            + f(p, aa) * p * past2[(1, a - 1)] //                 1 -> 0
            + f(p, aa) * p * past2[(5, a - 1)] //                 5 -> 0
            + f(p, b) * p * past2[(6, a - 1)]; //                 6 -> 0
        if a >= 3 {
            o[(0, a)] = o[(0, a)]
            + f(p, b) * p2 * past3[(2, a - 2)] //                 2 -> 0
            + (four * p3q + p4) * f(p, aa) * past4[(3, a - 2)] // 3 -> 0
            + f(p, b) * p2 * past3[(4, a - 2)] //                 4 -> 0
        }
        o[(1, a)] = n(p, b) * o[(0, a)] //                        0 -> 1
            + f(p, aa) * q * past1[(1, a)] //                     1 -> 1
            + f(p, b) * pq * past2[(2, a - 1)] //                 2 -> 1
            + f(p, aa) * p2q2 * past3[(3, a - 1)]; //             3 -> 1
        o[(6, a)] = f(p, b) * q * past1[(6, a - 1)]; //           6 -> 6
        if a >= 3 {
            o[(6, a)] = o[(6, a)] //                                 3 -> 6
                + f(p, aa) * p2q2 * past3[(3, a - 2)]
        }
        o[(5, a)] = f(p, aa) * p2q2 * past3[(3, a - 1)] //        3 -> 5
            + f(p, b) * pq * past2[(4, a - 1)] //                 4 -> 5
            + f(p, aa) * q * past1[(5, a)]; //                    5 -> 5
        o[(4, a)] = f(p, aa) * pq2 * past2[(3, a - 1)] //         3 -> 4
            + f(p, b) * q * past1[(4, a - 1)] //                  4 -> 4
            + n(p, aa) * o[(5, a)]; //                            5 -> 4
        o[(2, a)] = n(p, aa) * o[(1, a)] //                       1 -> 2
            + f(p, b) * q * past1[(2, a - 1)] //                  2 -> 2
            + f(p, aa) * pq2 * past2[(3, a - 1)] //               3 -> 2
            + n(p, b) * o[(6, a)]; //                             6 -> 2
        o[(3, a)] = n(p, b) * o[(2, a)] //                        2 -> 3
            + f(p, aa) * q2 * past1[(3, a)] //                    3 -> 3
            + n(p, b) * o[(4, a)] //                              4 -> 3
    }
}
