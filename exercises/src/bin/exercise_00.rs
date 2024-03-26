/// # Supported primitive types
///
/// Nothing needs to be done for this exercise, it is just to show the primitive types.
///

#[allow(unused_variables)]
fn exercise() {
    // Primitive types
    let bool_value = true; // OR let bool_value: bool = true;

    let u8_value: u8 = 20; // OR let u8_value = 20u8;
    let u16_value: u16 = 20; // OR let u16_value = 20u16;
    let u32_value: u32 = 20; // OR let u32_value = 20u32;
    let u64_value: u64 = 20; // OR let u64_value = 20u64;
    let u128_value: u128 = 20; // OR let u128_value = 20u128;
    let usize_value: usize = 20; // OR let usize_value = 20usize; takes 4 bytes on 32-bit systems and 8 bytes on 64-bit systems

    let i8_value: i8 = 20; // OR let i8_value = 20i8;
    let i16_value: i16 = 20; // OR let i16_value = 20i16;
    let i32_value = 20; // OR let i32_value: i32 = 20; OR let i32_value = 20i32; Default type for integers
    let i64_value: i64 = 20; // OR let i64_value = 20i64;
    let i128_value: i128 = 20; // OR let i128_value = 20i128;
    let isize_value: isize = 20; // OR let isize_value = 20isize; takes 4 bytes on 32-bit systems and 8 bytes on 64-bit systems

    let f32_value: f32 = 20.0; // OR let f32_value = 20f32;
    let f64_value = 20.0; // OR let f64_value: f64 = 20; OR let f64_value = 20f64; Default type for floats

    let str_value = "hello world"; // Reference to a string slice
    let char_value = 'a'; // OR let char_value: char = 'a';

    let array_type: [i32; 5] = [1, 2, 3, 4, 5]; // OR let array_type = [1, 2, 3, 4, 5]; Note that only if the elements are primitive types, the array itself can be treated as primitive.
}

fn main() {
    exercise();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        assert!(true);
    }
}
