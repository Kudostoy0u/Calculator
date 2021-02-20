#[path = "to_postfix.rs"]
mod to_postfix;
#[path = "solve.rs"]
mod solve;
pub fn eq(infix: Vec<String>) -> f64 {
  let postfix = to_postfix::convert(infix,Vec::new(),Vec::new(),0,"NaN".to_string());
  let mut rpn = postfix.0;
  for i in postfix.1 { rpn.push(i.1); };
  return solve::to_number(rpn);
}