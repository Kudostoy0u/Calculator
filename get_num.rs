#[path = "is_num.rs"]
mod is_num;
#[path = "display_error.rs"]
mod display_error;
// Get full number
pub fn find(e: Vec<String>,previous:String) -> String {
let mut num = "".to_string();
#[allow(unused_variables)]
let mut period_has_occured = false;
for i in 0..e.len() {
  let elem = &e[i];
  num.push_str(elem);
  // Factorial
  if (elem == "!" || elem == "s") && i <= e.len()-1 {
    continue;
  }
  // Unary
  if (elem == "-" || elem == "+" || elem == ".") && !is_num::parse(&previous) && i == 0 && &previous != ")" {
    if &e.get(i+1) == &None {
      display_error::error("Kalculater: Error parsing expression: Unary symbol has no succeeding number");
    } else { continue; }
  }
  if is_num::parse(elem) || (elem == "." && num.len() > 1 && !period_has_occured && i != e.len()-1 && is_num::parse(&e[i+1])) {
    if elem == "." {
      period_has_occured = true;
    }
    continue;
  } else {
    if num.len() > 1 {
    num = (&num[0..num.len()-1]).to_string(); 
    }
    if elem == "." && num.len() == 1 {
      display_error::error("Kalculater: Error parsing expression: Unexpected period");
    }
  break;
}
}
return num.to_string();
}