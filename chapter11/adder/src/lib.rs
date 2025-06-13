pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn should_pass_again() {
        let result = add(2, 5);
        assert_ne!(result, 9, "we are testing that the values are not equal");
        assert_eq!(result, 7, "we are testing that 2 + 5 = 7");
    }
}
