/// #  Functions
///
/// There is an issue in this code snippet! Can you find it?
///

fn exercise() -> i32 {
    // Do not change this function
    let value = 7;
    square(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), 49);
    }
}
