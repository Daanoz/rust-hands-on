/// # Enumerations
///
/// Enum can hold implementation like structs, add a method `switch_to_next`,
/// which moves to state from Green -> Yellow -> Red -> Green.
///

fn exercise() -> TrafficLight {
    // Do not change this function
    let light = TrafficLight::Green;
    let light = light.switch_to_next();
    light.switch_to_next()
}

#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
impl TrafficLight {
    fn switch_to_next(&self) -> TrafficLight {
        if self == &TrafficLight::Green {
            TrafficLight::Yellow
        } else if self == &TrafficLight::Yellow {
            TrafficLight::Red
        } else if self == &TrafficLight::Red {
            TrafficLight::Green
        } else {
            unreachable!()
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
        assert_eq!(exercise(), TrafficLight::Red);
    }
    #[test]
    fn test_states() {
        assert_eq!(TrafficLight::Green.switch_to_next(), TrafficLight::Yellow);
        assert_eq!(TrafficLight::Yellow.switch_to_next(), TrafficLight::Red);
        assert_eq!(TrafficLight::Red.switch_to_next(), TrafficLight::Green);
    }
}
