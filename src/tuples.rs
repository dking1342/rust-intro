// Tuples group together values of different types
// Max 12 elements

pub fn run(){
  // set tuple
  let person:(&str, &str, i8) = ("kavooce","usa",11);

  // tuples can be accessed by dot notation
  println!("{} is from {} and is {}", person.0, person.1, person.2);
}