use std::f64;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Dual {
    pub real: f64,
    pub dual: f64,
}

impl Dual {
    fn new(real: f64, dual: f64) -> Dual {
        Dual {
            real: real,
            dual: dual,
        }
    }
}

impl PartialEq for Dual {
    fn eq(&self, other: &Dual) -> bool {
        self.real == other.real && self.dual == other.dual
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, other: Self) -> Dual {
        Dual::new(self.real + other.real, self.dual + other.dual)
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, other: Self) -> Dual {
        Dual::new(self.real - other.real, self.dual - other.dual)
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, other: Self) -> Dual {
        Dual::new(
            self.real * other.real,
            self.real * other.dual + self.dual * other.real,
        )
    }
}

impl Div for Dual {
    type Output = Self;

    fn div(self, other: Dual) -> Dual {
        if other.real.abs() <= f64::EPSILON {
            panic!("Zero is an invalid denominator!");
        }
        Dual::new(
            self.real / other.real,
            (self.dual * other.real - self.real * other.dual) / (other.real * other.real),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Dual;

    #[test]
    fn test_add() {
        let d1 = Dual::new(1.0, 2.0);
        let d2 = Dual::new(3.0, 4.0);
        let d = d1 + d2;
        assert_eq!(d.real, 4.0);
        assert_eq!(d.dual, 6.0);
    }
}
