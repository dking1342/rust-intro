

pub fn run(){
  let age:u8 = 18;
  let check_id:bool = false;
  let knows_person_age: bool = true;

  if age >= 21 && check_id || knows_person_age {
    println!("bartender: what would you like to drink?");
  } else if age < 21 && check_id {
    println!("bartender: sorry you have to leave!");
  } else {
    println!("bartender: let me see your id");
  }

  let is_of_age = if age >= 21 { true } else { false };
  println!("is of age: {}", is_of_age);
}