use std::env;

pub fn run(){
  let args: Vec<String> = env::args().collect();
  println!("args: {:?}",args);
  
  let command = args[0].clone();
  println!("command: {:?}",command);

  let name = "kavooce";
  let status = "100%";

  if command == "target/debug/rust1" {
    println!("Hello {}, how are you?", name);
  } else if command == "status" {
    println!("Status is {}", status);
  } else {
    println!("that is not a command");
  }

}