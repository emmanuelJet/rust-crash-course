#![deny(clippy::all)]

// const MY_AGE: u8 = 24; // Constants are upper-cased and typed in snake case and should always have a default value

fn main() {
  // let full_name: &str = "Emmanuel Joseph (JET)"; // Immutable variable

  // String
  // let mut name: &str = "JET"; // Mutable variable
  // name = "JET";

  // Integer
  // let age: u32 = 24; // Other integer types include u or i 8, 16, 32, 64, 128
  // let _age = 20u8; // "_" before the variable name can be used to silent the comment error
  // let distance_in_km: f32 = 5.5; // The other floating point type is f64
  // let population: i32 = 62_000_000; // Other integer literals (data specifiers) include 0x123(Can be used for Hex color), Oo12, Ob1111, Ob1111_1010, ete

  // Integer Operator
  // let x: f64 = 5.5;
  // let y: f64 = 6.4;
  // let _sum_of_x_and_y: f64 = x + y;

  // Variable Shadowing
  // let full_name: &str = "JET"; // You can also change the data type i.e &str can be i32

  // Const
  // let my_age: u8 = MY_AGE; // A variable can be a constant but not the other way round

  // Tuples - Useful for storing information that are useful together
  let personal_data: (i32, &str) = (24, "JET");
  // let (age, name): (i32, &str) = personal_data; Combined fetching of Tuple
  let age: i32 = personal_data.0;
  let name: &str = personal_data.1;

  println!("Hello, {}!", name);
  println!("My age is {}.", age);
}
