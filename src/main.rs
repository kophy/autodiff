mod dual;

use dual::Dual;

fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    let d1 = Dual { r: 1.0, d: 2.0 };

    let d2 = Dual { r: 3.0, d: 4.0 };

    println!("{:?}", d1 + d2);
}
