#[allow(dead_code)]
// cannot assign twice to immutable variables
pub(crate) fn variables() {
    let x = 5;
    let mut y = 10;
    println!("{}, {}", x, y);

    y = 20;
    println!("{}, {}", x, y);

    // constants should have upper case names
    const Z: &str = "constant";

    println!("{}", Z);

}

// shadowing with different types not allowed for mutable variables
pub(crate) fn shadowing() {

    //allowed
    let x = 5;

    let x = x+5;
    let x = x*2;
    println!("{}", x);

    let y = "  ";
    let y = y.len();

    println!("{}", y)

    //not allowed

    // let mut y = "  ";
    // y = y.len();

}