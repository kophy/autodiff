use std::f64;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Dual {
    pub r: f64, // real part
    pub d: f64, // dual part
}

impl Dual {
    fn new(r: f64, d: f64) -> Dual {
        Dual { r: r, d: d }
    }
}

impl PartialEq for Dual {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.d == other.d
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.r + other.r, self.d + other.d)
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.r - other.r, self.d - other.d)
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.r * other.r, self.r * other.d + self.d * other.r)
    }
}

impl Div for Dual {
    type Output = Self;

    fn div(self, other: Dual) -> Self {
        if other.r.abs() <= f64::EPSILON {
            panic!("Zero is an invalid denominator!");
        }
        Self::new(
            self.r / other.r,
            (self.d * other.r - self.r * other.d) / (other.r * other.r),
        )
    }
}

impl ToString for Dual {
    fn to_string(&self) -> String {
        if self.d.is_sign_negative() {
            format!("{} - {}ε", self.r, self.d.abs())
        } else {
            format!("{} + {}ε", self.r, self.d)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dual;

    #[test]
    fn dual_basic() {
        let d1 = Dual::new(1.0, 2.0);
        let d2 = Dual::new(3.0, 4.0);
        assert_eq!(d1 + d2, Dual::new(4.0, 6.0));
        assert_eq!(d1 - d2, Dual::new(-2.0, -2.0));
        assert_eq!(d1 * d2, Dual::new(3.0, 10.0));
        assert_eq!(d1 / d2, Dual::new(1.0 / 3.0, 2.0 / 9.0));
    }

    #[test]
    #[should_panic]
    fn dual_panic() {
        let d1 = Dual::new(1.0, 2.0);
        let d2 = Dual::new(0.0, 4.0);
        assert_eq!(d1 / d2, Dual::new(0.0, 0.0));
    }

    #[test]
    fn dual_to_string() {
        assert_eq!(Dual::new(1.0, 2.0).to_string(), String::from("1 + 2ε"));
        assert_eq!(Dual::new(1.0, -2.0).to_string(), String::from("1 - 2ε"));
    }
}
