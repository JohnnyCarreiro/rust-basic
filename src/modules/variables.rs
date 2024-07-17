#![allow(dead_code)]

pub fn test_variables() {
  // Rust has fallows the immutability principle, witch means we must let explicit some variable could receive a new value;
  // Here, 'greeting' is declared as a constant and cannot be changed afterwards.
  let greeting: String = "Hello, world!".to_string();
  println!("{}", greeting);

  // Trying to change the value of 'greeting' results in a compile-time error.
  // Uncomment the following line to see the error:
  // greeting = "Goodbye, world!";

  // Rust also has mutable variables, which can be changed without explicitly declaring them as mutable.
  let mut greeting: &str = "Hello World";
  println!("{}", greeting);
  greeting = "Goodbye, world!";
  println!("{}", greeting);
}

pub fn primitive_variables() {
  // Integer types
  // For Integers we have signed values i8,i16,i32,i64 and arch isize
  // and also unsigned values are u8, u16, u32, u64 and usize;
  // The default value is i32.

  //Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.
  let x: i32 = -5;
  let y: u32 = 10;

  // Floating-point types
  // All float point are signed and it has sizes os f64 and f32, the default value is f64.
  #[allow(clippy::approx_constant)]
  let z: f64 = 3.14;

  // Boolean type
  let is_awesome: bool = true;

  println!(
    "x = {}, y = {}, z = {}, is_awesome = {}",
    x, y, z, is_awesome
  );

  // Char type
  // Rust's char type represents a Unicode Scalar Value. It's a 4-byte value.
  // It should be initialized with single quotes;
  let c: char = 'a';

  println!("c = {}", c);
  let z: char = 'ℤ'; // with explicit type annotation
  let heart_eyed_cat = '😻';
  println!("z = {}, heart_eyed_cat = {}", z, heart_eyed_cat);

  // Tuple type
  #[allow(clippy::approx_constant)]
  let tuple: (i32, f64, char) = (5, 3.14, 'a');
  let x: (i32, f64, u8) = (500, 6.4, 1);

  let five_hundred = x.0;

  let six_point_four = x.1;

  let one = x.2;
  println!("Tuple: {:?}", tuple);
  println!(
    "Tuple values: {}, {}, {}",
    five_hundred, six_point_four, one
  );

  // Array type
  // Arrays are imutable with fixed sizes
  let array: [i32; 5] = [1, 2, 3, 4, -5];
  println!("Array: {:?}", array);

  // String type
  // String is the most common string type. It has ownership over the contents of the string, stored in a heap-allocated buffer
  let string: String = "Hello, World!".to_string();
  println!("String: {}", string);

  // Slice type
  // The str type, also called a 'string slice', is the most primitive string type. It is usually seen in its borrowed form, &str. It is also the type of string literals, &'static str.
  let slice: &str = &string[0..5];
  println!("Slice: {}", slice);

  // Byte array type
  let bytes: [u8; 5] = [104, 101, 108, 108, 111];
  println!("Bytes: {:?}", bytes);

  // Reference type
  let reference = &string;
  println!("Reference: {}", reference);

  //
}

pub fn simple_math() {
  // addition
  let sum = 5 + 10;

  // subtraction
  let difference = 95.5 - 4.3;

  // multiplication
  let product = 4 * 30;

  // division
  let quotient = 56.7 / 32.2;
  let truncated = -5 / 3; // Results in -1

  // remainder
  let remainder = 43 % 5;
  println!("addition 5 + 10 = {sum}, difference 95.5 - 4.3 = {difference}, product 4 * 30 = {product}, quotient 56.7 / 32.3 = {quotient}, truncated -5 / 3 = {truncated} remainder 43 % 5 = {remainder}");
}

pub fn test_array() {
  use std::io;
  let array = [1, 2, 3, 4, 5];

  println!("Array: {:?}", array);
  println!("Please enter an array index.");

  let mut index: String = String::new();
  io::stdin()
    .read_line(&mut index)
    .expect("Unable to read input");
  let index: usize = index.trim().parse().expect("Unable to convert to integer!");

  let element = array[index];

  println!("Element in array is: {}", element);
}
