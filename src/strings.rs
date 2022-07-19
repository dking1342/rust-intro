// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
  // primitive string
  let _hello = "hello";

  // String
  let mut greeting = String::from("Hola ");
  println!("{}",greeting);

  // get length
  println!("Length: {}", greeting.len());
  
  // modify String
  greeting.push('A'); // pushes char
  greeting.push_str("migo!"); // pushes string
  println!("Append: {}", greeting);

  // capacity
  println!("Capacity: {}", greeting.capacity());

  // is empty?
  println!("is empty: {}", greeting.is_empty());

  // contains?
  println!("contains hola: {}", greeting.contains("Hola"));

  // replace
  println!("replace: {}", greeting.replace("Hola", "Howdy"));

  // loop through string by whitespace
  for word in greeting.split_whitespace() {
    println!("{}",word);
  }

  // create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("{}",s);
  
  // assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
  println!("{}",s);

}