/// # Borrowing
///
/// Adjust `read_first_word` function so it takes a reference to value,
/// and returns a reference to the first word of the value.
///
fn exercise() -> String {
    // Do not change this function
    let value = String::from("Hello world");
    let first_word = read_first_word(&value);
    assert_eq!(first_word, "Hello");
    value
}

fn read_first_word() {
    // Adjust the signature of this function to make the example compile
    value.split_once(' ').unwrap().0
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), String::from("Hello world"));
    }
}
