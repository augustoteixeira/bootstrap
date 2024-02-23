use core::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Log<T>(pub T);

#[allow(dead_code)]
impl Log<f32> {
    pub fn new(v: f32) -> Self {
        if v.ln().is_nan() {
            panic!("new with nan for v = {}", v);
        }
        Log(v.ln())
    }
    pub fn powi(self, n: i32) -> Self {
        if (self.0 * (n as f32)).is_nan() {
            panic!("powi with nan for self = {}, n = {}", self, n);
        }
        Log::<f32>(self.0 * (n as f32))
    }
    pub fn ln_to_float(self) -> f64 {
        if self.0.is_nan() {
            panic!("ln with nan for self = {}", self);
        }
        self.0 as f64
    }
    pub fn to_float(self) -> f64 {
        if self.0.abs() > 20.0 {
            panic!("Calculating f32 exp of too large a value: {}!", self.0);
        }
        self.0.exp() as f64
    }
}

impl Default for Log<f32> {
    fn default() -> Self {
        Log::<f32>(std::f32::NEG_INFINITY)
    }
}

impl Add for Log<f32> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.0.is_infinite() & other.0.is_infinite() {
            return Default::default();
        }
        let max = self.0.max(other.0);
        let min = self.0.min(other.0);
        //Self(max + (1.0 + (min - max).exp()).ln());

        let r = max + (1.0 + (min - max).exp()).ln();
        if r.is_nan() {
            panic!("nan when add({:}, {:})", self, other)
        }
        Self(r)
    }
}

impl Sub for Log<f32> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        if self.0.is_infinite() & other.0.is_infinite() {
            return Default::default();
        }
        let max = self.0.max(other.0);
        let min = self.0.min(other.0);
        //Self(max + (1.0 - (min - max).exp()).ln())
        let r = max + (1.0 - (min - max).exp()).ln();
        if r.is_nan() {
            panic!("nan when add({:}, {:})", self, other)
        }
        Self(r)
    }
}

//////////////// f64 //////////////////

#[allow(dead_code)]
impl Log<f64> {
    pub fn new(v: f32) -> Self {
        if v.ln().is_nan() {
            panic!("new with nan for v = {}", v);
        }
        Log((v as f64).ln())
    }
    pub fn powi(self, n: i32) -> Self {
        if (self.0 * (n as f64)).is_nan() {
            panic!("powi with nan for self = {}, n = {}", self, n);
        }
        Log::<f64>(self.0 * (n as f64))
    }
    pub fn ln(self) -> Self {
        if self.0.is_nan() {
            panic!("ln with nan for self = {}", self);
        }
        Log::<f64>(self.0.ln())
    }
    pub fn ln_to_float(self) -> f64 {
        if self.0.is_nan() {
            panic!("ln with nan for self = {}", self);
        }
        self.0 as f64
    }
    pub fn to_float(self) -> f64 {
        if self.0.abs() > 20.0 {
            panic!("Calculating f64 exp of too large a value: {}!", self.0);
        }
        self.0.exp()
    }
}

impl Default for Log<f64> {
    fn default() -> Self {
        Log::<f64>(std::f64::NEG_INFINITY)
    }
}

impl Add for Log<f64> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.0.is_infinite() & other.0.is_infinite() {
            return Default::default();
        }
        let max = self.0.max(other.0);
        let min = self.0.min(other.0);
        let r = max + (1.0 + (min - max).exp()).ln();
        if r.is_nan() {
            panic!("nan when add({:}, {:})", self, other)
        }
        Self(r)
    }
}

impl Sub for Log<f64> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let max = self.0.max(other.0);
        let min = self.0.min(other.0);
        Self(max + (1.0 - (min - max).exp()).ln())
    }
}

// impl<T> Default for Log<T> {
//     fn default() -> Self {
//         Log::<T>::new(std::f32::NEG_INFINITY)
//     }
// }

impl<T: core::fmt::Display> fmt::Display for Log<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "exp({})", self.0)
    }
}

impl<T: std::ops::Sub + Sub<Output = T>> Div for Log<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl<T: std::ops::Add + Add<Output = T>> Mul for Log<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl<T: std::cmp::PartialEq> PartialEq for Log<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: std::cmp::PartialOrd> PartialOrd for Log<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
