// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run(){
  // type and size needed
  let mut numbers:[i32;5] = [1,2,3,4,5];
  println!("{:?}",numbers);
  
  // single value
  println!("{:?}",numbers[0]);

  // reassign array value
  numbers[2] = 30;
  println!("{:?}",numbers);
  
  // get array length
  println!("Array length: {}",numbers.len());
  
  // arrays are stack allocated
  println!("Array occupies {} bytes",std::mem::size_of_val(&numbers));
  println!("Array occupies {} bytes",mem::size_of_val(&numbers));

  // get slice of array
  let slice:&[i32] = &numbers[0..2];
  println!("slice: {:?}",slice);

}