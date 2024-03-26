/// # Standard enumerations - Option
///
/// If the `integer_divided_by_two` function returns Some value, add its output to the sum.
/// There are multiple ways to solve this problem, can you think of other ways to solve it?
///

fn exercise() -> i32 {
    let mut sum = 0;

    for i in 1..=100 {
        let divided = integer_divided_by_two(i);
        todo!("If the `integer_divided_by_two` function returns Some value, add its output to the sum");
    }

    sum
}

fn integer_divided_by_two(value: i32) -> Option<i32> {
    // Do not change this function
    if value % 2 == 0 {
        Some(value / 2)
    } else {
        None
    }
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), 1275);
    }
}
