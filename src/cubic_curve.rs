use std::{
    cmp,
    ops::{Add, Div, Mul, Sub},
};

pub struct Cubic {
    pub curves: Vec<Curve>,
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum Curve {
    Float(f64),
    Int(i64),
}

impl Cubic {
    pub fn new(curves: Vec<Curve>) -> Cubic {
        Cubic { curves }
    }

    pub fn get_value(&self, time: Curve) -> Curve {
        let (mut start_gradient, mut end_gradient) = (Curve::default(), Curve::default());
        let (mut start, mut mid) = (0.0, 0.0);
        let mut end = 1.0;

        if time <= 0.0 {
            if self.curves[0] > 0.0 {
                start_gradient = self.curves[1] / self.curves[0];
            } else if self.curves[0] == 0.0 && self.curves[2] > 0.0 {
                start_gradient = self.curves[3] / self.curves[2];
            }
            return start_gradient * time;
        }

        if time >= 1.0 {
            if self.curves[2] < 1.0 {
                end_gradient = (self.curves[3] - 1.0) / (self.curves[2] - 1.0);
            } else if self.curves[2] == 1.0 && self.curves[0] < 1.0 {
                end_gradient = (self.curves[1] - 1.0) / (self.curves[0] - 1.0);
            }
            return end_gradient * (time - 1.0) + 1.0;
        }

        while start < end {
            mid = (start + end) / 2.0;
            let x_est = self.calculate(self.curves[0], self.curves[2], mid);
            if (time - x_est).abs() < 0.00001 {
                return self.calculate(self.curves[1], self.curves[3], mid);
            }
            if x_est < time {
                start = mid;
            } else {
                end = mid;
            }
        }

        self.calculate(self.curves[1], self.curves[3], mid)
    }

    pub fn calculate(&self, a: Curve, b: Curve, m: f64) -> Curve {
        a * 3.0 * (1.0 - m) * (1.0 - m) * m + b * 3.0 * (1.0 - m) * m * m + m * m * m
    }
}

impl Curve {
    pub fn new() -> Curve {
        Curve::Float(0.0)
    }

    pub fn abs(&self) -> Curve {
        match self {
            Curve::Float(f) => Curve::Float(f.abs()),
            Curve::Int(i) => Curve::Int(i.abs()),
        }
    }

    pub fn as_f64(&self) -> f64 {
        match self {
            Curve::Float(f) => *f,
            Curve::Int(i) => *i as f64,
        }
    }

    pub fn as_i64(&self) -> i64 {
        match self {
            Curve::Float(f) => f.trunc() as i64,
            Curve::Int(i) => *i,
        }
    }
}

impl Default for Curve {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Curve {}

impl PartialEq<f64> for Curve {
    fn eq(&self, other: &f64) -> bool {
        match self {
            Curve::Float(f) => f == other,
            Curve::Int(i) => *i as f64 == *other,
        }
    }
}

impl PartialOrd<f64> for Curve {
    fn partial_cmp(&self, other: &f64) -> Option<cmp::Ordering> {
        match self {
            Curve::Float(f) => f.partial_cmp(other),
            Curve::Int(i) => (*i as f64).partial_cmp(other),
        }
    }
}

impl Div for Curve {
    type Output = Curve;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Curve::Float(f1), Curve::Float(f2)) => Curve::Float(f1 / f2),
            (Curve::Int(i1), Curve::Int(i2)) => Curve::Int(i1 / i2),
            (Curve::Float(f1), Curve::Int(i2)) => Curve::Float(f1 / i2 as f64),
            (Curve::Int(i1), Curve::Float(f2)) => Curve::Float(i1 as f64 / f2),
        }
    }
}

impl Div<f64> for Curve {
    type Output = Curve;
    fn div(self, rhs: f64) -> Self::Output {
        match self {
            Curve::Float(f1) => Curve::Float(f1 / rhs),
            Curve::Int(i1) => Curve::Float(i1 as f64 / rhs),
        }
    }
}

impl Mul for Curve {
    type Output = Curve;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Curve::Float(f1), Curve::Float(f2)) => Curve::Float(f1 * f2),
            (Curve::Int(i1), Curve::Int(i2)) => Curve::Int(i1 * i2),
            (Curve::Float(f1), Curve::Int(i2)) => Curve::Float(f1 * i2 as f64),
            (Curve::Int(i1), Curve::Float(f2)) => Curve::Float(i1 as f64 * f2),
        }
    }
}

impl Mul<f64> for Curve {
    type Output = Curve;
    fn mul(self, rhs: f64) -> Self::Output {
        match self {
            Curve::Float(f1) => Curve::Float(f1 * rhs),
            Curve::Int(i1) => Curve::Float(i1 as f64 * rhs),
        }
    }
}

impl Sub for Curve {
    type Output = Curve;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Curve::Float(f1), Curve::Float(f2)) => Curve::Float(f1 - f2),
            (Curve::Int(i1), Curve::Int(i2)) => Curve::Int(i1 - i2),
            (Curve::Float(f1), Curve::Int(i2)) => Curve::Float(f1 - i2 as f64),
            (Curve::Int(i1), Curve::Float(f2)) => Curve::Float(i1 as f64 - f2),
        }
    }
}

impl Sub<f64> for Curve {
    type Output = Curve;
    fn sub(self, rhs: f64) -> Self::Output {
        match self {
            Curve::Float(f1) => Curve::Float(f1 - rhs),
            Curve::Int(i1) => Curve::Float(i1 as f64 - rhs),
        }
    }
}

impl Sub<Curve> for i64 {
    type Output = Curve;
    fn sub(self, rhs: Curve) -> Self::Output {
        match rhs {
            Curve::Float(f1) => Curve::Float(self as f64 - f1),
            Curve::Int(i1) => Curve::Float(self as f64 - i1 as f64),
        }
    }
}

impl Add for Curve {
    type Output = Curve;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Curve::Float(f1), Curve::Float(f2)) => Curve::Float(f1 + f2),
            (Curve::Int(i1), Curve::Int(i2)) => Curve::Int(i1 + i2),
            (Curve::Float(f1), Curve::Int(i2)) => Curve::Float(f1 + i2 as f64),
            (Curve::Int(i1), Curve::Float(f2)) => Curve::Float(i1 as f64 + f2),
        }
    }
}

impl Add<f64> for Curve {
    type Output = Curve;
    fn add(self, rhs: f64) -> Self::Output {
        match self {
            Curve::Float(f1) => Curve::Float(f1 + rhs),
            Curve::Int(i1) => Curve::Float(i1 as f64 + rhs),
        }
    }
}
