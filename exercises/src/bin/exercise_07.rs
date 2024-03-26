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

    // format!("{a}={c}")
    // format!("{b}={c}")
    // format!("{a}={d}")
    // format!("{b}={d}")
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
