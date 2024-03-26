/// # Using modules / public/ private
///
/// Private vs Public access in a mod. Until till now all our exercise have been running
/// in the same module (visibility scope), outside of the module, everything is private
/// by default, meaning that they can't be accessed from outside the module. A file is a
/// module, which could contain submodules denoted by the `mod` keyword.
/// Fix the code so that the `Model` struct is accessible from the `exercise` function.
///

fn exercise() -> i32 {
    let output = example::create_model();
    output.value
}

mod example {
    pub fn create_model() -> Model {
        Model { value: 5 }
    }

    struct Model {
        value: i32,
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
        assert_eq!(exercise(), 5);
    }
}
