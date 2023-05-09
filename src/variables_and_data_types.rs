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