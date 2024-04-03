/// # Structures + Ownership
///
/// Use the provided builder pattern methods to create an order.
/// Due to the ownership rules of rust, there is a unique way of making the builder pattern work. After
/// completing the question, try to update the shipping costs of the `OrderBuilder` after calling `build`.
/// You'll see this is impossible.
///

#[allow(unused)]
fn exercise() -> i32 {
    let cost = 100;
    let shipping = 10;
    let tax = 5;

    let order = OrderBuilder::new(cost).shipping(shipping).tax(tax).build();
    order.total_cost()
}

#[allow(unused)]
use order::*;
mod order {
    // Do not change this module (modules will be explained later)
    pub struct OrderBuilder {
        cost: i32,
        shipping: i32,
        tax: i32,
    }

    impl OrderBuilder {
        pub fn new(cost: i32) -> Self {
            Self {
                cost,
                shipping: 0,
                tax: 0,
            }
        }

        pub fn shipping(mut self, cost: i32) -> Self {
            self.shipping = cost;
            self
        }
        pub fn tax(mut self, cost: i32) -> Self {
            self.tax = cost;
            self
        }
        pub fn build(self) -> Order {
            Order {
                total_costs: self.cost + self.shipping + self.tax,
            }
        }
    }

    pub struct Order {
        total_costs: i32,
    }
    impl Order {
        pub fn total_cost(&self) -> i32 {
            self.total_costs
        }
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
        assert_eq!(exercise(), 115);
    }
}
