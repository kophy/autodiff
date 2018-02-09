mod dual;
mod function;

use dual::Dual;

fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    let d1 = Dual {
        real: 1.0,
        dual: 2.0,
    };

    let d2 = Dual {
        real: 3.0,
        dual: 4.0,
    };

    println!("{:?}", d1 + d2);
}
