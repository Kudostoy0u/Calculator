#[path = "display_error.rs"]
mod display_error;
pub fn tok(tokens: Vec<(String,bool)>,index: usize,mut istrue: bool) -> bool {
  if tokens.get(index+1) == None {
    return istrue;
  }
  let current = &tokens[index];
  if current.1 {
  let previous_ok = tokens[index-1].0.parse::<f64>();
  let next_ok = tokens[index+1].0.parse::<f64>();
  if !(previous_ok.is_ok() || next_ok.is_ok()) {
  display_error::error("Kalculater: Error parsing expression: Invalid equality");
  }
  let previous = previous_ok.unwrap();
  let next = next_ok.unwrap();
  istrue = match current.0.as_str() {
    "<=" => previous <= next,
    ">=" => previous >= next,
    "<" => previous < next,
    ">" => previous > next,
    "=" => previous == next,
    _ => {
    display_error::error("Kalculater: Unknown error, maybe an unexpected character?");     
    return false;
    }
  }
  }
  return tok(tokens,index+1,istrue);
}