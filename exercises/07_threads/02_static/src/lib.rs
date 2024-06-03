// TODO: Given a static slice of integers, split the slice into two halves and
//  sum each half in a separate thread.
//  Do not allocate any additional memory!
use std::thread::spawn;

pub fn sum(slice: &'static [i32]) -> i32 {
    let midpoint = slice.len() / 2;
    let bottom_half = &slice[..midpoint];
    let top_half = &slice[midpoint..];

    let bottom_worker = spawn(|| bottom_half.iter().sum());
    let top_worker = spawn(|| top_half.iter().sum());

    let bottom_result: i32 = bottom_worker.join().unwrap();
    let top_result: i32 = top_worker.join().unwrap();

    bottom_result + top_result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
