// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; //vec wasnt declared as mutable therefore it cannot be borrowed as mutable

    vec.push(88);

    vec //returns the vector
}

fn main() {
    let vec0 = vec![1, 2, 3];
    let vec1 = fill_vec(vec0); //call the fill_vec function to append a value to the vector
    println!("{:?}", vec1); //print the vector 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
