pub type Index = i32;

pub trait Memory {
    fn new(side: Index) -> Self;
    fn get_side(&self) -> Index;
    fn get(&self, x: Index, y: Index) -> bool;
    fn set(&mut self, x: Index, y: Index);
}
