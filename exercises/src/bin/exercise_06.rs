/// # Functions
///
/// There are two issues in this code snippet! Can you find them?
///

fn exercise() -> i32 {
    // Do not change this function
    let value = 7;
    divide(value)
}

fn divide(value: i32) {
    if value == 0 {
        0
    } else {
        value / value
    };
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), 1);
    }

    #[test]
    fn test_divide_0() {
        assert_eq!(divide(0), 0);
    }
}
