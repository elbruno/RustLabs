pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        let result = is_even(10);
        assert_eq!(true, result);
    }

    #[test]
    fn is_false_when_odd() {
        let result = is_even(11);
        assert_eq!(false, result);
    }
}