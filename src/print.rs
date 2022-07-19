pub fn run(){
  // print to console
  println!("hello from the print.rs file");

  // wrong- cannot print integer
  // println!(1);
  // correct
  println!("Number: {}", 1);

  // adding dynamic data
  println!("{} is from {}", "kavooce","africa");

  // positional arguments
  println!("{} is from {} and {} likes to {}", "kavooce","africa", "kavooce", "code");
  println!("{0} is from {1} and {0} likes to {2}", "kavooce","africa", "code");

  // named arguments
  println!("{name} likes to play {activity}", name="kavooce",activity="saxophone");

  // placeholder traits
  println!("Binary: {:b} Hex: {:x} Oct: {:o}",10,10,10);

  // placehoder for debug traits
  println!("{:?}",(12,true,"hello"));

  // basic math
  println!("10 + 10 = {}", 10 + 10);
}