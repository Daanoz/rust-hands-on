/// # Structures
///
/// Add a method to the struct returning the total cost of the order
///

fn exercise() -> i32 {
    // Do not change this function
    let order = Order {
        cost: 100,
        shipping: 10,
        tax: 5,
    };
    order.total_cost()
}

struct Order {
    cost: i32,
    shipping: i32,
    tax: i32,
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), 115);
    }
}
