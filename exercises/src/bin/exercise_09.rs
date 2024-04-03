/// # Borrowing
///
/// `read_string` function currently takes ownership of the value passed to it.
/// Lets try to adjust the read_string function (and maybe the caller side) so
/// that calling read_string with same value multiple times works.
///

#[allow(unused)]
fn exercise() -> String {
    // Do not change this function
    let value = String::from("Hello world");
    read_string(&value);
    read_string(&value);
    value
}

fn read_string(value: &String) {
    println!("{}", value);
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
