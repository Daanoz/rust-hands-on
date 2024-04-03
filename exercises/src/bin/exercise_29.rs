/// # Code generation
///
/// Try to add 3 oranges, 2 apples and 5 bananas to the basket. To make this work you need to add
/// macro to the Product struct.

fn exercise((orange, apple, banana): (Product, Product, Product)) -> f32 {
    let mut basket = Basket { items: vec![] };

    for _ in 0..3 {
        basket.add(orange.clone());
    }
    for _ in 0..2 {
        basket.add(apple.clone());
    }
    for _ in 0..5 {
        basket.add(banana.clone());
    }

    basket.print();
    basket.total()
}

#[derive(Debug, Clone)]
struct Product {
    #[allow(dead_code)]
    name: String,
    price: f32,
}

struct Basket {
    // No changes needed to this struct
    items: Vec<Product>,
}

impl Basket {
    // No changes needed to this struct
    fn add(&mut self, product: Product) {
        self.items.push(product);
    }

    fn print(&self) {
        println!("Items: {:?}", self.items);
    }

    fn total(self) -> f32 {
        self.items.into_iter().map(|product| product.price).sum()
    }
}

fn main() {
    exercise((
        Product {
            name: "Orange".to_string(),
            price: 0.5,
        },
        Product {
            name: "Apple".to_string(),
            price: 0.75,
        },
        Product {
            name: "Banana".to_string(),
            price: 0.40,
        },
    ));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert!(
            (exercise((
                Product {
                    name: "Orange".to_string(),
                    price: 0.5,
                },
                Product {
                    name: "Apple".to_string(),
                    price: 0.75,
                },
                Product {
                    name: "Banana".to_string(),
                    price: 0.40,
                },
            )) - 5.0)
                .abs()
                <= 10.0 * f32::EPSILON
        );
    }
}
