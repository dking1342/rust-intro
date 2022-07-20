// Vectors - Resizable arrays

use std::mem;

pub fn run(){
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20;

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get single val
  println!("Single Value: {}", numbers[0]);

  // Get vector length
  println!("Vector Length: {}", numbers.len());

  // Vectors are heap allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get Slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop & mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);

  let mut states: Vec<String> = vec![String::from("WI"),String::from("CA"),String::from("CO")];
  println!("states: {:?}", states);

  states[1] = String::from("HA");
  println!("states: {:?}", states);
  
  states.push(String::from("MI"));
  println!("states: {:?}", states);
  
  states.pop();
  println!("states: {:?}", states);
  
  println!("states: {:?}", states[0]);
  
  println!("states len: {:?}", states.len());

  let slice_string: &[String] = &states[1..3];
  println!("slice: {:?}",slice_string);
  
  for x in states.iter() {
    println!("state: {}",x);
  }
  
  for x in states.iter_mut(){
    *x = String::from("X");
  }
  println!("mut vector: {:?}",states);

}