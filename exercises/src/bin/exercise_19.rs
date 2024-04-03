/// # Control flow: loops
///
/// Implement a loop that sums the numbers 1 to 100.
///

fn exercise() -> i32 {
    let mut sum = 0;

    for i in 1..=100 {
        sum += i;
    }

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
