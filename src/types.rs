/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run(){
  // default is "i32"
  let _x = 1;

  // default is "f64"
  let _y = 2.5;

  // add explicit type
  let _z: i64 = 4545454545454;

  // find max size
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // boolean
  let is_active = true;
  let _is_running:bool = false;
  println!("{:?}",(_x,_y,_z,is_active));

  // get bool from expression
  let is_greater_10 = 10 > 5;
  let is_greater_20 = 20 > 100;
  println!("{:?}",(is_greater_10, is_greater_20));

  // char
  let a1 = 'a';
  let face = '\u{1F600}';
  println!("{:?}",(a1,face));
}