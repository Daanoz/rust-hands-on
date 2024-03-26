/// # Control flow: if
///
/// Implement a function `sign` that takes an integer and returns 1 if the number
/// is positive, -1 if the number is negative, and 0 if the number is zero.
///

fn exercise() -> () {
    // Do not change this function
    println!("Sign for 5 is {}", sign(5));
    println!("Sign for 25 is {}", sign(25));
    println!("Sign for -5 is {}", sign(-5));
    println!("Sign for 0 is {}", sign(0));
}

fn sign(value: i32) -> i32 {
    todo!("implement sign function here")
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        for i in 1..=10 {
            assert_eq!(sign(i), 1);
        }
    }
    #[test]
    fn test_zero() {
        assert_eq!(sign(0), 0);
    }
    #[test]
    fn test_negative() {
        for i in -10..=-1 {
            assert_eq!(sign(i), -1);
        }
    }
}
