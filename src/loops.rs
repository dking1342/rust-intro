// Loops - Used to iterate until a condition is met

pub fn run(){
  let mut count = 0;

  // infinite loop
  loop {
    count += 1;
    println!("count: {}", count);

    if count == 20 {
      break;
    }
  }

  let mut counter = 0;
  // while loop
  while counter <= 100 {

    if counter % 15 == 0 {
      println!("fizzbuzz");
    } else if counter % 3 == 0 {
      println!("fizz");
    } else if counter % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}",counter);
    }
    
    // increment
    counter += 1;
  }
  
  for x in 0..100 {
    if x % 15 == 0 {
      println!("fizzbuzz");
    } else if x % 3 == 0 {
      println!("fizz");
    } else if x % 5 == 0 {
      println!("buzz");
    } else {
      println!("{}",x);
    }    
  }
}