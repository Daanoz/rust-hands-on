/// # Closures
///
/// Call the apply function twice with:
/// - a closure that sums `a` + `b`
/// - a closure that multiplies previous result * `b`
///

fn exercise(a: i32, b: i32) -> i32 {
    let sum = apply(|a, b| a + b, a, b);
    apply(|a, b| a * b, sum, b)
}

fn apply<F: Fn(i32, i32) -> i32>(func: F, a: i32, b: i32) -> i32 {
    // Do not change this function
    func(a, b)
}

fn main() {
    println!("{}", exercise(5, 7));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(5, 7), 84);
        assert_eq!(exercise(2, 3), 15);
        assert_eq!(exercise(12, -7), -35);
    }
}
