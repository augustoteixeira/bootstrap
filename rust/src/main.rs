use astro_float::ctx::Context;
//use astro_float::expr;
use astro_float::BigFloat;
use astro_float::Consts;
use astro_float::RoundingMode;

use grid::*;

const PR: usize = 60;
const RM: RoundingMode = RoundingMode::Up;
const LOG_MULTIPLE: f64 = 1.5;

fn n(p: &BigFloat, n: usize) -> BigFloat {
    let q: BigFloat = BigFloat::from(1.0).sub(p, PR, RM);
    q.powi(n, PR, RM)
}

fn f(p: &BigFloat, n: usize) -> BigFloat {
    let q: BigFloat = BigFloat::from(1.0).sub(p, PR, RM);
    BigFloat::from(1.0).sub(&q.powi(n, PR, RM), PR, RM)
}

fn main() {
    let ctx = Context::new(
        PR,
        RM,
        Consts::new().expect("Contants cache initialized"),
    );
    let (_, _, mut cc) = ctx.to_raw_parts();
    let m_min = 2;
    let m_table = 2;
    let m_max = 8;
    for m in m_min..=m_max {
        let p = BigFloat::from(0.5).powi(m, PR, RM);
        //let q = BigFloat::from(1.0).sub(&p, PR, RM);
        let p_f: f64 = 2.0_f64.powf(-(m as f64));
        let a: usize = (LOG_MULTIPLE * (1.0 / p_f).ln() / p_f) as usize;
        let mut n00: Vec<BigFloat> = Vec::with_capacity(a);
        let mut n01: Vec<BigFloat> = Vec::with_capacity(a);
        let mut n10: Vec<BigFloat> = Vec::with_capacity(a);
        let mut n11: Vec<BigFloat> = Vec::with_capacity(a);
        let mut n20: Vec<BigFloat> = Vec::with_capacity(a);
        let mut n21: Vec<BigFloat> = Vec::with_capacity(a);

        let mut current0: &mut Vec<BigFloat>;
        let mut past10: &mut Vec<BigFloat>;
        let mut past20: &mut Vec<BigFloat>;
        let mut current1: &mut Vec<BigFloat>;
        let mut past11: &mut Vec<BigFloat>;
        let mut past21: &mut Vec<BigFloat>;

        for _ in 0..a {
            n00.push(BigFloat::from(0.0));
            n01.push(BigFloat::from(0.0));
            n10.push(BigFloat::from(0.0));
            n11.push(BigFloat::from(0.0));
            n20.push(BigFloat::from(0.0));
            n21.push(BigFloat::from(0.0));
        }

        let tab_h = 6;
        let tab_w = 5;
        let mut table: Grid<BigFloat> = Grid::new(tab_w, tab_h);

        n20[1] = p.clone();
        n21[1] = p.mul(&n(&p, 1), PR, RM);
        table[1][1] = p.clone();
        // println!(
        //     "p = {}, p_f = {}, t11 = {}, q = {}",
        //     p,
        //     p_f,
        //     table[1][1].ln(PR, RM, &mut cc),
        //     q
        // );

        for s in 3..a {
            match s % 3 {
                0 => {
                    current0 = &mut n00;
                    current1 = &mut n01;
                    past10 = &mut n20;
                    past11 = &mut n21;
                    past20 = &mut n10;
                    past21 = &mut n11;
                }
                1 => {
                    current0 = &mut n10;
                    current1 = &mut n11;
                    past10 = &mut n00;
                    past11 = &mut n01;
                    past20 = &mut n20;
                    past21 = &mut n21;
                }
                2 => {
                    current0 = &mut n20;
                    current1 = &mut n21;
                    past10 = &mut n10;
                    past11 = &mut n11;
                    past20 = &mut n00;
                    past21 = &mut n01;
                }
                _ => unreachable!(),
            }
            modified(&p, s, current0, current1, past10, past11, past20, past21);

            if m == m_table {
                for l in 0..tab_w {
                    let sub: usize = ((s as i64) - (l as i64)) as usize;
                    if sub < tab_h {
                        //println!("a = {a}, s = {s}, l = {l}, sub = {sub}");
                        table[l][sub] = current0[l].clone();
                    }
                }
            }

            if s == a - 1 {
                let mut sum: BigFloat = BigFloat::from(0.0);
                for l in 0..a {
                    sum = sum.add(&current0[l], PR, RM);
                }
                println!(
                    "a = {a}, m = {m}, p = {p}, -p * log(sum) = {}",
                    p.mul(&BigFloat::from(-1.0), PR, RM).mul(
                        &sum.ln(PR, RM, &mut cc),
                        PR,
                        RM
                    )
                );
            }
        }
        if m == m_table {
            // print table
            for b in (0..tab_h).rev() {
                for a in 0..tab_w {
                    print!("M[{a}, {b}] = ");
                    let value = table[a][b].ln(PR, RM, &mut cc).mul(
                        &BigFloat::from(-1.0),
                        PR,
                        RM,
                    );
                    print!("{}, ", value);
                }
                println!("");
            }
        }
    }
}

fn modified(
    p: &BigFloat,
    s: usize,
    o0: &mut Vec<BigFloat>,
    o1: &mut Vec<BigFloat>,
    l0: &mut Vec<BigFloat>,
    l1: &mut Vec<BigFloat>,
    _b0: &mut Vec<BigFloat>,
    b1: &mut Vec<BigFloat>,
) {
    let q = BigFloat::from(1.0).sub(p, PR, RM);
    for a in 1..s {
        let b: usize = s - a;
        o0[a] = f(&p, b).mul(&l0[a - 1], PR, RM).add(
            &p.mul(&b1[a - 1], PR, RM),
            PR,
            RM,
        );
        o1[a] = q.mul(&f(&p, a), PR, RM).mul(&l1[a], PR, RM).add(
            &n(&p, b).mul(&o0[a], PR, RM),
            PR,
            RM,
        )
    }
}
