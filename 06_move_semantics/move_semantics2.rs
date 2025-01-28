fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(vec0.clone()); /* vec0 is moved into the fill_vec function,
        so vec0 is no longer accessible after this point. to fix, we can clone vec0 */

        assert_eq!(vec0, [22, 44, 66]); //the error is here: vec0 is not accessible therefore cannot use it
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
