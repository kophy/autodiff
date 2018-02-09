mod dual;
mod function;

use dual::Dual;
use function::{constant, sin, pow, exp};

fn diff(x: f64) -> f64 {
    // y = sin(sqrt(e^x + 3) / 2)
    let x = Dual::new(x, 1.0);
    let y = sin(pow(exp(x) + constant(3.0), 0.5) / constant(2.0));
    y.dual
}

fn main() {
    let mut vec: Vec<f64> = Vec::new();
    for i in -3..4 {
        vec.push(i as f64 * 0.2);
    }
    for x in &vec {
        println!("x = {:.2}, dy/dx = {:.5}", x, diff(*x));
    }
}
