/// # Standard enumerations - Result
///
/// Take the input as a string and convert it to an integer using the `int_value` function.
/// If the input is not a number, return 0.
///
use std::num::ParseIntError;

fn exercise(input: &str) -> i32 {
    todo!();
}

#[allow(dead_code)]
fn int_value(value: &str) -> Result<i32, ParseIntError> {
    // Do not change this function
    value.parse()
}

fn main() {
    println!("{}", exercise("5"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise("5"), 5);
        assert_eq!(exercise("five"), 0);
    }
}
