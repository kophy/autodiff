use std::f64;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Dual {
    pub real: f64,
    pub dual: f64,
}

impl Dual {
    pub fn new(r: f64, d: f64) -> Dual {
        Dual { real: r, dual: d }
    }
}

impl PartialEq for Dual {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.dual == other.dual
    }
}

impl Add for Dual {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (r1, d1) = (&self.real, &self.dual);
        let (r2, d2) = (&other.real, &other.dual);
        Self::new(r1 + r2, d1 + d2)
    }
}

impl Sub for Dual {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (r1, d1) = (&self.real, &self.dual);
        let (r2, d2) = (&other.real, &other.dual);
        Self::new(r1 - r2, d1 - d2)
    }
}

impl Mul for Dual {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (r1, d1) = (&self.real, &self.dual);
        let (r2, d2) = (&other.real, &other.dual);
        Self::new(r1 * r2, r1 * d2 + d1 * r2)
    }
}

impl Div for Dual {
    type Output = Self;

    fn div(self, other: Dual) -> Self {
        let (r1, d1) = (&self.real, &self.dual);
        let (r2, d2) = (&other.real, &other.dual);
        Self::new(r1 / r2, (d1 * r2 - r1 * d2) / (r2 * r2))
    }
}

impl ToString for Dual {
    fn to_string(&self) -> String {
        if self.dual.is_sign_negative() {
            format!("{} - {}ε", self.real, self.dual.abs())
        } else {
            format!("{} + {}ε", self.real, self.dual)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dual;

    #[test]
    fn test_dual_basic() {
        let d1 = Dual::new(1.0, 2.0);
        let d2 = Dual::new(3.0, 4.0);
        assert_eq!(d1 + d2, Dual::new(4.0, 6.0));
        assert_eq!(d1 - d2, Dual::new(-2.0, -2.0));
        assert_eq!(d1 * d2, Dual::new(3.0, 10.0));
        assert_eq!(d1 / d2, Dual::new(1.0 / 3.0, 2.0 / 9.0));
    }

    #[test]
    #[should_panic]
    fn test_div_panic() {
        let d1 = Dual::new(1.0, 2.0);
        let d2 = Dual::new(0.0, 4.0);
        assert_eq!(d1 / d2, Dual::new(0.0, 0.0));
    }

    #[test]
    fn test_dual_to_string() {
        let d1 = Dual::new(1.0, 2.0);
        let d2 = Dual::new(1.0, -2.0);
        assert_eq!(d1.to_string(), String::from("1 + 2ε"));
        assert_eq!(d2.to_string(), String::from("1 - 2ε"));
    }
}
