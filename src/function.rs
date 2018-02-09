use dual::Dual;

// d(u^n)/dx = n * u^(n - 1) * du/dx
pub fn pow(u: &Dual, n: f64) -> Dual {
    Dual::new(u.real.powf(n), n * u.real.powf(n - 1.0) * u.dual)
}

// d(sin(u))/dx = cos(u) * du/dx
pub fn sin(u: &Dual) -> Dual {
    Dual::new(u.real.sin(), u.real.cos() * u.dual)
}

// d(cos(u))/dx = -sin(u) * du/dx
pub fn cos(u: &Dual) -> Dual {
    Dual::new(u.real.cos(), -u.real.sin() * u.dual)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use super::Dual;
    use super::{cos, pow, sin};

    #[test]
    fn test_pow() {
        // u = x, y = u^2, dy/dx = 2u * du/dx
        // x = 3.0, dy/dx = 6.0
        let u = Dual::new(3.0, 1.0);
        assert!((pow(&u, 2.0).dual - 6.0).abs() < 0.0001_f64);
    }

    #[test]
    fn test_sin() {
        // u = x, y = sin(u), dy/dx = cos(u) * du/dx
        // x = pi/3, dy/dx = 0.5
        let u = Dual::new(PI / 3.0, 1.0);
        assert!((sin(&u).dual - 0.5).abs() < 0.0001_f64);
    }

    #[test]
    fn test_cos() {
        // u = x, y = cos(u), dy/dx = -sin(u) * du/dx
        // x = pi/6, dy/dx = -0.5
        let u = Dual::new(PI / 6.0, 1.0);
        assert!((cos(&u).dual + 0.5).abs() < 0.0001_f64);
    }
}
