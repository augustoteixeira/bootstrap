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
            if nw && se {
                if updated == false {
                    if !(ne && sw) {
                        updated = true;
                    }
                }
                grid.set(x, y);
                grid.set(x + 1, y + 1);
            }
            if ne && sw {
                if updated == false {
                    if !(nw && se) {
                        updated = true;
                    }
                }
                grid.set(x + 1, y);
                grid.set(x, y + 1);
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
