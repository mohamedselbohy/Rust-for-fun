use std::ops::Add;
pub fn add<T: Add<Output = T>>(left: T, right: T) -> T {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_test() {
        let result = add(2, 3);
        assert_eq!(result, 5);
    }
}
