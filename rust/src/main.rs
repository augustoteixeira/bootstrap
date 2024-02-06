use grid::*;

const LOG_MULTIPLE: f64 = 1.5;

fn n(p: f64, n: i32) -> f64 {
    (1.0 - p).powi(n)
}

fn f(p: f64, n: i32) -> f64 {
    1.0 - (1.0 - p).powi(n)
}

fn main() {
    let m_min = 4;
    //let m_table = 2;
    let m_max = 12;
    for m in m_min..=m_max {
        let p: f64 = (2.0 as f64).powi(-m);
        // println!(
        //     "p = {}, log(1/p) = {}, log(1/p)/p = {}, all = {}",
        //     p,
        //     (1.0 / p).ln(),
        //     (1.0 / p).ln() / p,
        //     (LOG_MULTIPLE * (1.0 / p).ln() / p)
        // );
        let a: usize = (LOG_MULTIPLE * (1.0 / p).ln() / p) as usize;
        // let mut n00: Vec<f64> = Vec::with_capacity(a);
        // let mut n01: Vec<f64> = Vec::with_capacity(a);
        // let mut n10: Vec<f64> = Vec::with_capacity(a);
        // let mut n11: Vec<f64> = Vec::with_capacity(a);
        // let mut n20: Vec<f64> = Vec::with_capacity(a);
        // let mut n21: Vec<f64> = Vec::with_capacity(a);
        let mut n0: Grid<f64> = Grid::new(7, a);
        let mut n1: Grid<f64> = Grid::new(7, a);
        let mut n2: Grid<f64> = Grid::new(7, a);
        let mut n3: Grid<f64> = Grid::new(7, a);

        let mut current: &mut Grid<f64>;
        let mut past1: &mut Grid<f64>;
        let mut past2: &mut Grid<f64>;
        let mut past3: &mut Grid<f64>;

        // //let tab_h = 6;
        // //let tab_w = 5;
        // //let mut table: Grid<f64> = Grid::new(tab_w, tab_h);

        for k in 0..7 {
            for s in 0..a {
                n0[(k, s)] = 0.0;
                n1[(k, s)] = 0.0;
                n2[(k, s)] = 0.0;
                n3[(k, s)] = 0.0;
            }
        }

        n2[(0, 1)] = p * n(p, 0);
        n2[(1, 1)] = p * n(p, 1);
        n2[(2, 1)] = p * n(p, 2);
        n2[(3, 1)] = p * n(p, 3);

        // n20[1] = p;
        // n21[1] = p * p.n(1);
        // //table[1][1] = p;
        // // println!(
        // //     "p = {}, p_f = {}, t11 = {}, q = {}",
        // //     p,
        // //     p_f,
        // //     table[1][1].ln(PR, RM, &mut cc),
        // //     q
        // // );

        for s in 3..a {
            match s % 4 {
                0 => {
                    current = &mut n0;
                    past1 = &mut n3;
                    past2 = &mut n2;
                    past3 = &mut n1;
                }
                1 => {
                    current = &mut n1;
                    past1 = &mut n0;
                    past2 = &mut n3;
                    past3 = &mut n2;
                }
                2 => {
                    current = &mut n2;
                    past1 = &mut n1;
                    past2 = &mut n0;
                    past3 = &mut n3;
                }
                3 => {
                    current = &mut n3;
                    past1 = &mut n2;
                    past2 = &mut n1;
                    past3 = &mut n0;
                }
                _ => unreachable!(),
            }
            modified(p, s, &mut current, &mut past1, &mut past2, &mut past3);

            //     if m == m_table {
            //         for l in 0..tab_w {
            //             let sub: usize = ((s as i64) - (l as i64)) as usize;
            //             if sub < tab_h {
            //                 //println!("a = {a}, s = {s}, l = {l}, sub = {sub}");
            //                 table[l][sub] = current0[l].clone();
            //             }
            //         }
            //     }

            if s == a - 1 {
                let mut sum: f64 = 0.0;
                let mut max: f64 = 0.0;
                for l in 0..a {
                    if (max < current[(0, l)]) {
                        max = current[(0, l)]
                    }
                    sum += current[(0, l)];
                }
                println!(
                    "p = {}, size = {}, -p * log(diag sum) = {}, m = {}",
                    p,
                    a,
                    -p * sum.ln(),
                    m
                );
            }

            // if m == m_table {
            //     // print table
            //     for b in (0..tab_h).rev() {
            //         for a in 0..tab_w {
            //             print!("M[{a}, {b}] = ");
            //             let value = table[a][b].ln(PR, RM, &mut cc).mul(
            //                 &f64::from(-1.0),
            //                 PR,
            //                 RM,
            //             );
            //             print!("{}, ", value);
            //         }
            //         println!("");
            //     }
        }
    }
}

fn modified(
    p: f64,
    s: usize,
    o: &mut Grid<f64>,
    p1: &mut Grid<f64>,
    p2: &mut Grid<f64>,
    p3: &mut Grid<f64>,
) {
    let q = 1.0 - p;
    for a in 1..s {
        let b: i32 = (s - a) as i32;
        o[(0, a)] = p * (p2[(1, a - 1)] + p2[(5, a - 1)] + p2[(6, a - 1)])
            + f(p, b) * p1[(0, a - 1)];
        o[(1, a)] = q * f(p, a as i32) * p1[(1, a)]
            + n(p, b) * o[(0, a)]
            + p * p2[(2, a - 1)];
        o[(5, a)] = q * f(p, a as i32) * p1[(5, a)] + p * p2[(4, a - 1)];
        if a < 3 {
            o[(6, a)] = 0.0;
        } else {
            o[(6, a)] = q * f(p, b) * p1[(6, a - 1)] + p * p * p3[(3, a - 2)];
        }
        o[(2, a)] = q * f(p, b) * p1[(2, a - 1)]
            + p * q * p2[(3, a - 1)]
            + q * n(p, a as i32) * o[(1, a)]
            + q * n(p, b) * o[(6, a)];
        o[(4, a)] = q * f(p, b) * p1[(4, a - 1)]
            + p * q * p2[(3, a - 1)]
            + q * n(p, a as i32) * o[(5, a)];
        o[(3, a)] = q * q * f(p, a as i32) * p1[(3, a)]
            + q * n(p, b) * o[(2, a)]
            + q * n(p, b) * o[(4, a)];
    }
}
