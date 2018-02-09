use dual::Dual;

// d(c)/dx = 0
pub fn constant(c: f64) -> Dual {
    Dual::new(c, 0.0)
}

// d(u^n)/dx = n * u^(n - 1) * du/dx
pub fn pow(u: Dual, n: f64) -> Dual {
    Dual::new(u.real.powf(n), n * u.real.powf(n - 1.0) * u.dual)
}

// d(e^u)/dx = e^u * du/dx
pub fn exp(u: Dual) -> Dual {
    Dual::new(u.real.exp(), u.real.exp() * u.dual)
}

// d(ln(u))/dx = 1/u * du/dx
pub fn ln(u: Dual) -> Dual {
    Dual::new(u.real.exp(), (1.0 / u.real) * u.dual)
}

// d(sin(u))/dx = cos(u) * du/dx
pub fn sin(u: Dual) -> Dual {
    Dual::new(u.real.sin(), u.real.cos() * u.dual)
}

// d(cos(u))/dx = -sin(u) * du/dx
pub fn cos(u: Dual) -> Dual {
    Dual::new(u.real.cos(), -u.real.sin() * u.dual)
}

// d(tan(u))/dx = (sec(u))^2 * du/dx = (1/cos(u))^2 * du/dx
pub fn tan(u: Dual) -> Dual {
    Dual::new(u.real.tan(), 1.0 / (u.real.cos()).powf(2.0) * u.dual)
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use super::Dual;
    use super::{cos, exp, ln, pow, sin, tan};

    #[test]
    fn test_pow() {
        // u = x, y = u^2, dy/dx = 2x
        // x = 2.0, dy/dx = 4.0
        let x = 2.0;
        let u = Dual::new(x, 1.0);
        assert!((pow(u, 2.0).dual - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_exp() {
        // u = x, y = e^u, dy/dx = e^x
        // x = 2.0, dy/dx = e^2
        let x = 2.0;
        let u = Dual::new(x, 1.0);
        assert!((exp(u).dual - (2.0f64).exp()).abs() < 1e-10);
    }

    #[test]
    fn test_ln() {
        // u = x, y = ln(u), dy/dx = 1/x
        // x = 2.0, y = 0.5
        let x = 2.0;
        let u = Dual::new(x, 1.0);
        assert!((ln(u).dual - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_sin() {
        // u = x, y = sin(u), dy/dx = cos(x)
        // x = pi/3, dy/dx = 0.5
        let x = PI / 3.0;
        let u = Dual::new(x, 1.0);
        assert!((sin(u).dual - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_cos() {
        // u = x, y = cos(u), dy/dx = -sin(x)
        // x = pi/6, dy/dx = -0.5
        let x = PI / 6.0;
        let u = Dual::new(x, 1.0);
        assert!((cos(u).dual + 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_tan() {
        // u = x, y = tan(u), dy/dx = 1 / cos(x)^2
        // x = pi / 3, dy/dx = 4
        let x = PI / 3.0;
        let u = Dual::new(x, 1.0);
        assert!((tan(u).dual - 4.0).abs() < 1e-10);
    }
}
