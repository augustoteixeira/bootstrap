use core::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Alg<T>(pub T);

impl<T: std::default::Default> Default for Alg<T> {
    fn default() -> Self {
        Alg::<T>(Default::default())
    }
}

impl<T: core::fmt::Display> fmt::Display for Alg<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

impl<T: std::ops::Add + Add<Output = T>> Add for Alg<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl<T: std::ops::Sub + Sub<Output = T>> Sub for Alg<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl<T: std::ops::Div + Div<Output = T>> Div for Alg<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0)
    }
}

impl<T: std::ops::Mul + Mul<Output = T>> Mul for Alg<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl<T: std::cmp::PartialEq> PartialEq for Alg<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: std::cmp::PartialOrd> PartialOrd for Alg<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
