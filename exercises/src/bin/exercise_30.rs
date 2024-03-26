/// # References and lifetimes
///
/// Same exercise as exercise 29 but now you need to add lifetimes to the Basket struct.
/// Do not clone the products. Try to add 3 oranges, 2 apples and 5 bananas to the basket.

fn exercise((orange, apple, banana): (Product, Product, Product)) -> f32 {
    let mut basket = Basket { items: vec![] };

    // basket.add(Product);

    basket.print();
    basket.total()
}

#[derive(Debug)]
struct Product {
    // No changes needed to this struct
    #[allow(dead_code)]
    name: String,
    price: f32,
}

struct Basket {
    items: Vec<Product>,
}

impl Basket {
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
