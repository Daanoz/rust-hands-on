/// # Variables
///
/// In this case we are trying to mutate a variable that is declared as immutable.
///

fn exercise() -> i32 {
    let mut output = 7;

    // Do not change below this line __________________________________________
    println!("The value of output is: {}", output);
    output = 5;
    println!("The value of output is: {}", output);
    output
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
