// Characters (`char`)

fn main() {
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: Analogous to the example before, declare a variable called `your_character`
    // below with your favorite character.
    // Try a letter, try a digit (in single quotes), try a special character, try a character
    // from a different language than your own, try an emoji 😉

    //let your_character = 'M'; //trying a letter ; (Alphabetical!)
    //let your_character = '2'; //trying a number ; (Numerical!)
    //let your_character = '('; //trying special character (Neither alphabetic nor numeric!)
    //let your_character = 'م'; //trying a character from a different language (Alphabetical!)
    //I didnt expect that one to be alphabetical. thought we'd get neither.
    //let your_character = '私'; //so lets try one more langauge: Japanese ; (Alphabetical!)
    let your_character = '😊'; //trying an emoji ; (Neither alphabetic nor numeric!)

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
