/// # Tuples + Destructuring
///
/// In `exercise()` use get_input to get a tuple of 5 values, consisting of integers
/// and strings. Return the sum of all integers and the concatenation of all strings.
/// Hint: with `format!("{str1}{str2}")` you can concatenate two strings.
///

fn exercise() -> (i32, String) {
    let (a, b, c, d, e) = get_input();
    (a + c + e, format!("{}{}", b, d))
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
