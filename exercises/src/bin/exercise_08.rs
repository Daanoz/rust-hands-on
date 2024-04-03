/// # Ownership
///
/// `read_string` function takes ownership of the value passed to it.
/// How can we adjust the code so that this works again?
///

#[allow(unused)]
fn exercise() -> String {
    let value = String::from("Hello world");
    let value = read_string(value);
    value
}

fn read_string(value: String) -> String {
    // Do not change this function
    println!("{}", value);
    value
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), String::from("Hello world"));
    }
}
