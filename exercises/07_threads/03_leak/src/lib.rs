// TODO: Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread::spawn;

pub fn sum(v: Vec<i32>) -> i32 {
    let static_vec: &'static mut [i32] = Vec::leak(v);

    let midpoint = static_vec.len() / 2;
    let bottom_half = static_vec[0..midpoint].to_vec();
    let top_half = static_vec[midpoint..].to_vec();

    let bottom_worker = spawn(move || bottom_half.iter().sum());
    let top_worker = spawn(move || top_half.iter().sum());

    let bottom_result: i32 = bottom_worker.join().unwrap();
    let top_result: i32 = top_worker.join().unwrap();

    bottom_result + top_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
