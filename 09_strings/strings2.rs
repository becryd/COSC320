// TODO: Fix the compiler error in the `main` function without changing this function.
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

//compiler error: mismatch types
//the function parameter expects type &str but it got String, so the argument is incorrect
//to fix, add & when passing the argument
fn main() {
    let word = String::from("green"); // Don't change this line.

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
