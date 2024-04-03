/// # Traits
///
/// A trait represents a set of methods, defined for an unknown type: `Self`
/// They can access other methods that have `Self` as a parameter.
///
/// In this exercise, you will implement a trait for a struct.
/// Implement the `Area` trait for the `Square` and `Circle` structs. The value
/// of the struct is the side of the square or the diameter of the circle.
/// The output of the `exercise` function should be the difference between the
/// area of the square and the area of the circle.
/// Hint: PI is available as `core::f32::consts::PI`.

fn exercise(a: f32) -> f32 {
    let square = Square { value: a };
    let circle = Circle { value: a };

    square.area() - circle.area()
}

struct Square {
    value: f32,
}
struct Circle {
    value: f32,
}
impl Area for Square {
    fn area(&self) -> f32 {
        self.value * self.value
    }
}
impl Area for Circle {
    fn area(&self) -> f32 {
        let radius = self.value / 2.0;
        core::f32::consts::PI * radius * radius
    }
}

trait Area {
    fn area(&self) -> f32;
}

fn main() {
    println!("{}", exercise(20.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        assert!((exercise(5.0) - 5.3650455).abs() <= f32::EPSILON);
        assert!((exercise(15.0) - 48.285416).abs() <= f32::EPSILON);
        assert!((exercise(115.0) - 2838.1084).abs() <= f32::EPSILON);
        assert!((exercise(20.0) - 85.84073).abs() <= f32::EPSILON);
    }
}
