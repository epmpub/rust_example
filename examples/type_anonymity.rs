// `F` must implement `Fn` for a closure which takes no
// inputs and returns nothing - exactly what is required
// for `print`.
fn apply<F>(mut f: F) where
    F: FnMut() {
    f();
}

fn main() {
    let x = 7;

    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);

    apply(print);

    println!("{number:<5}    {number1:<5}", number=1,number1=2);
    println!("{number:<5}    {number1:<5}", number=100,number1=20);
    println!("{number:<5}    {number1:<5}", number=1000,number1=2000);

}
