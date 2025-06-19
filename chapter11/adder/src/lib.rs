pub mod new_module;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = adder::add(arg, 9);
///
/// assert_eq!(14, answer);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    println!("yoyoyoy {right}");
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
        assert_eq!(result, 7, "expected value 5, got res");
    }

    #[test]
    fn another_test() -> Result<(), String> {
        let result = add(5, 10);
        if result == 15 {
            Ok(())
        } else {
            Err(String::from("This shouldn't have happened"))
        }
    }
}

#[cfg(test)]
mod sample {
    #[test]
    fn dance_with_me() {
        println!("yellowwww... i'm dancing");
    }
}

