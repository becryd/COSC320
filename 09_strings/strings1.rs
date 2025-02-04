// TODO: Fix the compiler error without changing the function signature.

//compiler error: mismatch types
fn current_favorite_color() -> String {
    /* the issue is, "blue" is a reference not a string and this function expects a string return
    add .to_string() to fix */
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
