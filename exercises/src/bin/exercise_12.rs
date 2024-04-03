/// # Tuples
///
/// Replace the todo! macro with the correct code to return the value 5 from from the tuple.
///

fn exercise() -> i32 {
    let tuple = (1, 2, 3, 5, 13, 21, 34);

    tuple.3
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), 5);
    }
}
