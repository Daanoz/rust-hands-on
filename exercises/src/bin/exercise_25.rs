/// # Generics
///
/// Generics is a way of generalizing types and functionalities to broader cases.
/// This is extremely useful in many situations, where code duplication would be
/// necessary otherwise.
///
/// Create the `get_first` function, that takes a tuple pair as an argument and
/// returns the first element of the pair.
///

fn exercise() -> String {
    let pair_a = (5, 16);
    let pair_b = (2.0, 33.0);
    let pair_c = ("hello", "world");

    format!(
        "first={}:{}:{}",
        get_first(pair_a),
        get_first(pair_b),
        get_first(pair_c)
    )
}

fn get_first<T>(pair: (T, T)) -> T {
    pair.0
}

fn main() {
    println!("{}", exercise());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), "first=5:2:hello");
    }
}
