use super::memory::{Index, Memory};

pub struct ByteArray {
    side: Index,
    data: Vec<bool>,
}

impl Memory for ByteArray {
    fn new(side: Index) -> Self {
        if side <= 0 {
            panic!("Size non-positive")
        }
        ByteArray {
            side,
            data: vec![false; (side * side).try_into().unwrap()],
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
        return unsafe { *self.data.get_unchecked(index) };
    }
    fn set(&mut self, x: Index, y: Index) {
        if x < 0 || x >= self.side {
            return;
        }
        if y < 0 || x >= self.side {
            return;
        }
        let index: usize = (x + y * self.side) as usize;
        self.data[index] = true;
    }
}
