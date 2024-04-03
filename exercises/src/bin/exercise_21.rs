/// # Control flow: pattern matching
///
/// Implement a make_sound method for the Animal enum, which returns the sound of the animal.
/// Try to use pattern matching to implement the method.
///

fn exercise() {
    // Do not change this function
    println!("Dog sound: {}", Animal::Dog.make_sound());
    println!("Cat sound: {}", Animal::Cat.make_sound());
    println!("Cow sound: {}", Animal::Cow.make_sound());
}

#[derive(Debug, PartialEq)]
enum Animal {
    Dog,
    Cat,
    Cow,
}
impl Animal {
    fn make_sound(&self) -> &str {
        match self {
            Animal::Dog => "Woof",
            Animal::Cat => "Meow",
            Animal::Cow => "Moo",
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
    fn test_animal_sounds() {
        let dog = Animal::Dog;
        assert_eq!(dog.make_sound(), "Woof");

        let cat = Animal::Cat;
        assert_eq!(cat.make_sound(), "Meow");

        let cow = Animal::Cow;
        assert_eq!(cow.make_sound(), "Moo");
    }
}
