pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// create unit test for the function add
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
