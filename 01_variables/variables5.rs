fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    /* the error is a type mismatch. to fix I'll just overwrite the previous variable (called shadowing) */
    let number : i32 = 3; //stating the datatype is optional
    println!("Number plus two is: {}", number + 2);
}
