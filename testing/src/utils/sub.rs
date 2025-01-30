use std::ops::Sub;
pub fn sub<T: Sub<Output = T>>(left: T, right: T) -> T {
    left - right
}
#[cfg(test)]
mod subtests {
    use std::u64;

    use super::*;
    #[test]
    pub fn test_i32() {
        let result = sub(5, 2);
        assert_eq!(result, 3);
    }
    #[test]
    pub fn sub_test_f64() {
        let result = sub(5.0, 2.0);
        assert_eq!(result, 3.0);
    }
    #[test]
    pub fn sub_test_u64() {
        let result = sub(u64::MAX, u64::MAX - 5);
        assert_eq!(result, 5, "2^64 - 1 - 2^64 + 1 + 5  is not returning 5");
    }
}
