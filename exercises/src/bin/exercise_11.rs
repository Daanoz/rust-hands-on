/// # Borrowing + Mutability
///
/// In this case we are trying to mutate a variable that is declared somewhere else.
/// Can you change the code so that the `times_two` function multiplies the input
/// value by 2? Hint: refer to exercise_01 to see how to add mutability to a variable.
///
fn exercise() -> i32 {
    let mut value = 5;
    times_two(&mut value);
    value
}

fn times_two(value: &mut i32) {
    *value *= 2; // This line does not require any changes!
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), 10);
    }
}
