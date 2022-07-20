// Enums are types which have a few definite values

enum Movement {
  Up,
  Down,
  Left,
  Right,
}

fn move_avatar(m: Movement) {
  match m {
    Movement::Up => println!("Avatar is moving up"),
    Movement::Down => println!("Avatar is moving down"),
    Movement::Left => println!("Avatar is moving left"),
    Movement::Right => println!("Avatar is moving right"),
  }
}

pub fn run(){

  let avatar1 = Movement::Up;
  let avatar2 = Movement::Left;
  let avatar3 = Movement::Down;
  let avatar4 = Movement::Right;

  move_avatar(avatar1);
  move_avatar(avatar2);
  move_avatar(avatar3);
  move_avatar(avatar4);
}