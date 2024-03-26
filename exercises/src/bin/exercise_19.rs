/// # Control flow: loops
///
/// Implement a loop that sums the numbers 1 to 100.
///

fn exercise() -> i32 {
    let mut sum = 0;

    todo!("Sum the numbers from 1 to 100 and store the result in the variable `sum` here");

    sum
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result() {
        assert_eq!(exercise(), 5050);
    }
}
