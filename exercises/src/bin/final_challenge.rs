/// # Final challenge
///
/// Complete the `challenge` function that takes a raw data file with the orders of a day separated
/// by a semicolon. Each day is a new line with the following format:
/// ```
/// Orders day 1: 3 apples, 2 bananas; 5 bananas, 2 apples, 12 pears; 1 apples, 1 pears
/// ```
/// Our store only sells `apples`, `bananas`, and `pears`, but there is a limited supply of each fruit,
/// there are 30 apples, 35 bananas, and 40 pears every day to be sold.
///
/// The function should return a tuple with:
/// - The first entry the sum of the day numbers we ran out stock of any of the fruits.
/// - The second entry can be calculated with:
///     - Find highest quantity per fruit per day.
///     - Multiply these three numbers to get a single value per day.
///     - Sum all days to get the final answer.
fn challenge(data: &str) -> (i32, i32) {
    (0, 0)
}

fn main() {
    let data = include_str!("../data/challenge_data.txt");
    challenge(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge() {
        assert_eq!(
            challenge(include_str!("../data/challenge_data.txt")),
            (3536, 114752)
        );
    }
}
