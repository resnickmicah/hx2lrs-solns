// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

use std::thread::ScopedJoinHandle;

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint = v.len() / 2;
    let bottom_half = &v[..midpoint];
    let top_half = &v[midpoint..];
    let mut bottom_result: i32 = 0;
    let mut top_result: i32 = 0;

    std::thread::scope(|scope| {
        let bottom_worker: ScopedJoinHandle<'_, i32> = scope.spawn(|| bottom_half.iter().sum());
        let top_worker: ScopedJoinHandle<'_, i32> = scope.spawn(|| top_half.iter().sum());

        bottom_result = bottom_worker.join().unwrap();
        top_result = top_worker.join().unwrap();
    });
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
