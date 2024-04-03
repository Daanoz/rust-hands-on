/// # Scopes
///
/// Using the same variable name in different scopes is allowed in Rust. How to tell the compiler
/// that the variable in the inner scope is different from the variable in the outer scope?
///

#[allow(unused_mut, unused_variables)]
fn exercise() -> i32 {
    let mut output = 7; // Do not change this line
    {
        println!("The value of output is: {}", output);
        let output = 5; // <-- how to tell the compiler that this is a different variable?
        println!("The value of output is: {}", output);
    }
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
        assert_eq!(exercise(), 7);
    }
}
