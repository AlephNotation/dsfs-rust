use num::{Float, Num};

//let height_weight_age = [70, 170, 40 ];  // years

// let grades = vec![95, 80, 75, 62];

fn vector_add<T: Num + Copy>(v: &[T], w: &[T]) -> Vec<T>{
    v.iter().zip(w).map(|(a, b)| *a + *b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(vector_add(&vec![0, 1, 2], &vec![1, 2, 3]), vec![1, 3, 5]);
        assert_eq!(
            vector_add(&vec![0.0, 2.0, 4.0], &vec![1.0, 2.0, 3.0]),
            vec![1.0, 4.0, 7.0]
        );
    }

}