/// # Tuples + Destructuring
///
/// In `exercise()` use get_input to get a tuple of 5 values, consisting of integers
/// and strings. Return the sum of all integers and the concatenation of all strings.
/// Hint: with `format!("{str1}{str2}")` you can concatenate two strings.
///

fn exercise() -> (i32, String) {
    todo!("Call get_input and return the sum of all i32 and the concatenation of all Strings")
}

fn get_input() -> (i32, String, i32, String, i32) {
    (1, String::from("Hello"), 2, String::from("World"), 3)
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), (6, "HelloWorld".to_string()));
    }
}
