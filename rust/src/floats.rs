use crate::alg::*;

impl<T> Alg<T> {}

#[allow(dead_code)]
impl Alg<f32> {
    pub fn new(v: f32) -> Self {
        Alg(v)
    }
    pub fn powi(self, n: i32) -> Self {
        Alg::<f32>(self.0.powi(n))
    }
    pub fn ln_to_float(self) -> f64 {
        self.0.ln() as f64
    }
    pub fn to_float(self) -> f64 {
        self.0 as f64
    }
}

#[allow(dead_code)]
impl Alg<f64> {
    pub fn new(v: f32) -> Self {
        Alg(v as f64)
    }
    pub fn powi(self, n: i32) -> Self {
        Alg::<f64>(self.0.powi(n))
    }
    pub fn ln_to_float(self) -> f64 {
        self.0.ln()
    }
    pub fn to_float(self) -> f64 {
        self.0
    }
}
