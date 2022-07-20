// Functions - Used to store blocks of code for re-use

pub fn run(){
  greeting("hola", "kavooce");
  
  // bind function vals to vars
  let get_sum = add(5,1);
  println!("sum: {}", get_sum);

  // closure
  let n3: i32 = 10;
  let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
  println!("csum: {}",add_nums(3,4));

}

fn greeting(greet:&str, name: &str){
  println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}