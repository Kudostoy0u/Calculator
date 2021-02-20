use std::io;
use std::io::*;
pub fn pause() {
  let mut input = String::new();
  print!("\x1b[1;36mPress enter to continue: \x1b[0m");
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).expect("error: unable to read user input");
  
}
pub fn read(display: &str) -> String {
  let mut input = String::new();
  print!("\x1b[1;33m{}\x1b[0m",display);
  io::stdout().flush().unwrap();
  io::stdin().read_line(&mut input).expect("error: unable to read user input");
  print!("\x1B[2J\x1B[1;1H");
  println!("\x1b[1;35mExpression\x1b[0m \x1b[1;33m{}\x1b[0m \x1b[1;35m->\x1b[0m",&input.trim());
  return input;
}