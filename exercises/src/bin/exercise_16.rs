/// # Structures
///
/// Add a method `have_equal_cost_without_tax` that take the sum of cost and shipping
/// of two orders and return true if they are equal, otherwise false
///

#[allow(unused_mut)]
fn exercise() -> bool {
    // Do not change this function
    let order1 = Order {
        cost: 100,
        shipping: 10,
        tax: 5,
    };
    let order2 = Order {
        cost: 105,
        shipping: 5,
        tax: 10,
    };

    Order::have_equal_cost_without_tax(&order1, &order2)
}

struct Order {
    cost: i32,
    shipping: i32,
    #[allow(unused)]
    tax: i32,
}

impl Order {
    fn cost_without_tax(&self) -> i32 {
        self.cost + self.shipping
    }
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(exercise(), true);
    }
}
