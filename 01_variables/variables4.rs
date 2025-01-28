// TODO: Fix the compiler error.
fn main() {
    /* the issue is that we're trying to change an immutable variable. */
    let mut x = 3; //make it mutable to fix
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
