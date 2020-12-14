use std::io;
use std::io::Write;

fn main() {
  let name: String = input("What's your name?\n");
  println!("Hello, {}!\n", name.trim());
  let num1: i32 = int_input("Type in an integer: ");
  let num2: f64 = float_input("Type in a float: ");
  
  println!("The numbers you typed where: {} and {}.", num1, num2);
}

fn int_input(text: &str) -> i32 {
  match input(text).trim().parse::<i32>() {
    Ok(ans) => {
      ans
    }
    Err(e) => {
      println!("Error: {:?}", e);
      1
    }
  }
}

fn float_input(text: &str) -> f64 {
  match input(text).trim().parse::<f64>() {
    Ok(ans) => {
      ans
    }
    Err(e) => {
      println!("Error: {:?}", e);
      1.0
    }
  }
}

fn input(text: &str) -> String {
  let mut user_input = String::new();
  
  print!("{}", text);
  io::stdout().flush().unwrap();
  
  match io::stdin().read_line(&mut user_input) {
    Ok(_) => {
      user_input
    }
    Err(_) => {
      String::from("ERROR")
    }
  }
}