fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        let second = numbers.1; //gets the second element in the tuple using its index, 1

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");

        /* To test, try: rustc --test filename.rs */
    }
}
