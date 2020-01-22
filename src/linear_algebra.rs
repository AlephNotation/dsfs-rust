use num::{Float, Num};

//let height_weight_age = [70, 170, 40 ];  // years

// let grades = vec![95, 80, 75, 62];

// add two vectors
fn vector_add<T: Num + Copy>(v: &[T], w: &[T]) -> Vec<T>{
    v.iter().zip(w).map(|(a, b)| *a + *b).collect()
}

// subtract two vectors
fn vector_subtract<T: Num + Copy>(v: &[T], w: &[T]) -> Vec<T>{
    v.iter().zip(w).map(|(a, b)| *a - *b).collect()
}


// reducer so the following are easier
fn reduce<T, F>(vs: &[Vec<T>], mut f: F) -> Option<Vec<T>>
where
    T: Num + Copy,
    F: FnMut(&[T], &[T]) -> Vec<T>,
{
    match vs.len() {
        0 => None,
        // it's a little silly to clone this, could maybe return None?
        1 => Some(vs[0].clone()),
        2 => Some(f(&vs[0], &vs[1])),
        _ => {
            let mut accum = f(&vs[0], &vs[1]);
            for v in vs[2..].iter() {
                accum = f(&accum, v);
            }
            Some(accum)
        }
    }
}

fn vector_sum<T: Num + Copy>(vs: &[Vec<T>]) -> Option<Vec<T>> {
    reduce(vs, vector_add)
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

    #[test]
    fn test_subtract(){
        assert_eq!(vector_subtract(&vec![1, 2, 3], &vec![0, 1, 2]), vec![1, 1, 1]);
        assert_eq!(vector_subtract(&vec![0.0, 1.0, 2.0], &vec![2.0, 4.0, 6.0]),
         vec![-2.0, -3.0, -4.0])
    }

    fn test_sum() {
        assert_eq!(vector_sum(&(Vec::<Vec<i32>>::new())), None);
        assert_eq!(vector_sum(&vec![vec![0, 1, 2]]), Some(vec![0, 1, 2]));
        assert_eq!(
            vector_sum(&vec![
                vec![0, 1, 2, 3, 4, 5],
                vec![1, 2, 3, 4],
                vec![2, 3, 4, 5, 5],
            ]),
            Some(vec![3, 6, 9, 12])
        )

    }


}