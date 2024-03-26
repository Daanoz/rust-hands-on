/// # Variables
///
/// Alternative solution to the problem in exercise_01.rs
///

fn exercise() -> i32 {
    let output = 7; // Do not change this line

    println!("The value of output is: {}", output);
    output = 5;
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
