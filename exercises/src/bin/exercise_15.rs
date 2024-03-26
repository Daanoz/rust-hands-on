/// # Structures
///
/// Add a method `apply_discount_to_cost` to the struct that allows applying a discount to the cost of the order
///

#[allow(unused_mut)]
fn exercise() -> i32 {
    // Do not change this function
    let mut order = Order {
        cost: 100,
        shipping: 10,
        tax: 5,
    };
    order.apply_discount_to_cost(10);

    let Order {
        cost,
        shipping,
        tax,
    } = order; // Destructuring of the order
    cost + shipping + tax
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
        assert_eq!(exercise(), 105);
    }
}
