use std::collections::{HashMap, HashSet};

/// # Iterators
///
/// Use various collection types to perform operations on the given data.
/// This time, use the `iter` method to iterate over the collections.
///

fn vector_sum(vector: Vec<i32>) -> i32 {
    todo!("Return the sum of the vector");
}

fn hashset_product(hashset: HashSet<i32>) -> i32 {
    todo!("Return the product of the hashset");
}

fn hashmap_sum(hashmap: HashMap<&str, Vec<i32>>) -> HashMap<&str, i32> {
    todo!("Return the sum of the hashmap values per key");
}

fn main() {
    let vector = Vec::from([1, 2, 3, 1]);
    let hash_set = HashSet::from([1, 2, 4, 2]);
    let hash_map = HashMap::from([("odd", vec![1, 3]), ("even", vec![2, 4])]);

    println!("The sum of the vector is: {:?}", vector_sum(vector));
    println!("The sum of the hashset is: {:?}", hashset_product(hash_set));
    println!("The sum of the hashmap is: {:?}", hashmap_sum(hash_map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_sum() {
        assert_eq!(vector_sum(vec![1, 2, 3, 1]), 7);
    }

    #[test]
    fn test_hashset_product() {
        assert_eq!(hashset_product(HashSet::from([1, 2, 4, 2])), 8);
    }

    #[test]
    fn test_hashmap_sum() {
        let hash_map = HashMap::from([("odd", vec![1, 3]), ("even", vec![2, 4])]);
        assert_eq!(
            hashmap_sum(hash_map),
            HashMap::from([("odd", 4), ("even", 6)])
        );
    }
}
