use std::process;
pub fn error(message: &str) {
  println!("\x1b[0;31m{}\x1b[0m",message);
  process::exit(0x0100);
}