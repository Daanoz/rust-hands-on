/// # Ownership
///
/// Given the following code, which of the lines 13-16 are invalid? And why?
/// And why are the others valid?
///

#[allow(unused)]
fn exercise() -> String {
    // Do not change this function
    let a = 5;
    let b = a;
    let c = String::from("five");
    let d = c;

    // format!("{a}={c}") // Invalid: borrow of moved value `c`
    // format!("{b}={c}") // Invalid: borrow of moved value `c`
    format!("{a}={d}") // Valid: as a is a primitive type, it has a copy trait,
                       // meaning that b is not taking ownership but is a copy of a
                       // format!("{b}={d}") // Valid: as a is a primitive type, it has a copy trait,
                       // meaning that b is not taking ownership but is a copy of a
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), String::from("5=five"));
    }
}
