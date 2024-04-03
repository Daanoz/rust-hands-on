/// # Variables
///
/// The type of a variable cannot be changed, even not if mutable. How can we fix the code below
/// when we still want to assign the integer to variable `output`?
///

#[allow(unused_mut)] // extra: after solving the problem, remove this line and observe the warning
fn exercise() -> i32 {
    let mut output = "seven"; // Do not change this line

    println!("The value of output is: {}", output);
    let output = 5;
    println!("The value of output is: {}", output);
    output // Do not change this line
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
