// Structs - Used to create custom data types

// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Struct
struct Colors(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // construct person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  // get full name
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  // set last name
  fn set_last_name(&mut self, last:&str){
    self.last_name = last.to_string();
  }

  // name to tuple
  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}


pub fn run(){
  let mut red = Color {
    red: 255,
    green: 0,
    blue: 0
  };
  println!("red color: {} {} {}",red.red, red.green, red.blue);
  
  // reassign 
  red.red = 200;
  println!("red color: {} {} {}",red.red, red.green, red.blue);
  
  let mut blue = Colors(0, 255, 0);
  println!("blue color: {} {} {}", blue.0, blue.1, blue.2);
  
  blue.1 = 200;
  println!("blue color: {} {} {}", blue.0, blue.1, blue.2);


  let mut p = Person::new("jack","jogger");
  println!("person {}", p.full_name());
  p.set_last_name("walker");
  println!("person {}", p.full_name());
  println!("person tuple {:?}", p.to_tuple());
}