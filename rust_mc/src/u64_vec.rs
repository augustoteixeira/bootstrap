use super::memory::{Index, Memory};

pub struct U64Array {
    pub next_value: u64,
    pub side: Index,
    pub data: Vec<u64>,
}

impl U64Array {
    pub fn set_next_value(&mut self, n: u64) {
        if n == u64::MAX {
            panic!("Invalid next value");
        }
        self.next_value = n;
    }

    pub fn get_value(&self, x: Index, y: Index) -> u64 {
        if x < 0 || x >= self.side {
            return u64::MAX;
        }
        if y < 0 || x >= self.side {
            return u64::MAX;
        }
        let index: usize = (x + y * self.side) as usize;
        return unsafe { *self.data.get_unchecked(index) };
    }
}

impl Memory for U64Array {
    fn new_filled_with_false(side: Index) -> Self {
        if side <= 0 {
            panic!("Size non-positive")
        }
        U64Array {
            next_value: 0,
            side,
            data: vec![u64::MAX; (side * side).try_into().unwrap()],
        }
    }

    fn get_side(&self) -> Index {
        self.side
    }

    fn get(&self, x: Index, y: Index) -> bool {
        if x < 0 || x >= self.side {
            return false;
        }
        if y < 0 || x >= self.side {
            return false;
        }
        let index: usize = (x + y * self.side) as usize;
        return unsafe { *self.data.get_unchecked(index) < u64::MAX };
    }

    fn set(&mut self, x: Index, y: Index) {
        if x < 0 || x >= self.side {
            return;
        }
        if y < 0 || x >= self.side {
            return;
        }
        let index: usize = (x + y * self.side) as usize;
        if self.data[index] == u64::MAX {
            self.data[index] = self.next_value;
        }
    }
}
