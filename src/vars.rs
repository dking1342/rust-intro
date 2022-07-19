// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
  // set a variable
  let name = "kavooce";
  println!("my name is {}", name);
  
  // 
  let age = 30;
  // cannot change variable
  // age = 31;
  println!("my name is {} and I am {} years old", name, age);

  // mutable variable
  let mut mut_age = 20;
  println!("my name is {} and I am {} years old", name, mut_age);
  mut_age = 21;
  println!("my name is {} and I am {} years old", name, mut_age);
  
  // define constant
  // uppercase conventions
  const ID:i32 = 001;
  println!("ID: {}",ID);

  // assign multiple vars
  let ( my_name, my_age ) = ("kavooce", 10);
  println!("{} is {}", my_name, my_age);

}