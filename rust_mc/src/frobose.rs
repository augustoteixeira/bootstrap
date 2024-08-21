use super::memory::Memory;

const FROBOSE_LOG_MULTIPLIER: f64 = 2.0;

pub fn frobose_droplet_size(p: f64) -> usize {
    let log_inv_p = -p.ln();
    return (FROBOSE_LOG_MULTIPLIER * log_inv_p / p) as usize;
}

pub fn frobose_step(grid: &mut impl Memory) -> bool {
    // we think of the origin as being in the left bottom corner
    let mut nw: bool;
    let mut ne: bool;
    let mut sw: bool;
    let mut se: bool;
    let side = grid.get_side();
    let mut updated = false;
    for y in 0..(side - 1) {
        for x in 0..(side - 1) {
            (nw, ne, sw, se) = (
                grid.get(x, y + 1),
                grid.get(x + 1, y + 1),
                grid.get(x, y),
                grid.get(x + 1, y),
            );
            //println!("{:},{:},{:},{:},{:},{:}", x, y, nw, ne, sw, se);
            match (nw, ne, sw, se) {
                (true, true, true, false) => {
                    updated = true;
                    grid.set(x + 1, y);
                }
                (true, true, false, true) => {
                    updated = true;
                    grid.set(x, y);
                }
                (true, false, true, true) => {
                    updated = true;
                    grid.set(x + 1, y + 1);
                }
                (false, true, true, true) => {
                    updated = true;
                    grid.set(x, y + 1);
                }
                _ => {} // if none of the above happended, we don't do anything
            }
        }
    }
    return updated;
}

pub fn frobose_run(grid: &mut impl Memory) -> usize {
    let mut i = 0;
    let mut updated;
    loop {
        updated = frobose_step(grid);
        i += 1;
        if !updated {
            break;
        }
    }
    return i;
}

#[cfg(test)]
mod tests {
    use crate::bool_vec::ByteArray;
    use crate::frobose::frobose_step;
    use crate::memory::Memory;

    #[test]
    fn insufficient_frobose_update() {
        let mut grid = ByteArray::new_filled_with_false(4);
        grid.set(0, 0);
        grid.set(1, 1);
        frobose_step(&mut grid);
        assert_eq!(grid.get(1, 0), false);
        assert_eq!(grid.get(0, 1), false);
    }

    #[test]
    fn sufficient_frobose_update() {
        for j in 0..4 {
            let mut grid = ByteArray::new_filled_with_false(4);
            for i in 0..4 {
                if i != j {
                    grid.set(i / 2, i % 2);
                }
            }
            frobose_step(&mut grid);
            assert_eq!(grid.get(j / 2, j % 2), true);
        }
    }
}
