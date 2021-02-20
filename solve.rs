#[path = "display_error.rs"]
mod display_error;
fn has(ops: [&str;6],elem: &str) -> bool {
  for i in ops.iter() {
    if i == &elem {
      return true;
    }
  }
  return false;
}
fn factorial(num: &f64,acc:f64) -> f64 {
    if num <= &0.0 {
        return acc;
    }
    return factorial(&(num-1.0),acc*num)
}
fn parse(toparse: &str) -> f64 {
  let num = toparse.parse::<f64>();
  if num.is_ok() {
    return num.unwrap();
  } else {
    display_error::error(format!("Kalculator: unknown character: \"{}\"",toparse).as_str());  
    return 666.0; 
  }
}
pub fn to_number(rpn: Vec<String>) -> f64 {
let mut stack: Vec<f64> = Vec::new();
let mut l = 0;  
if rpn.len() == 0 {
  display_error::error("Kalculater: Error parsing expression: No input");
}
if rpn.last().unwrap() == "(" {
  display_error::error("Kalculater: Error parsing expression: Unbalanced parenthesis");
}
loop {
if &rpn.get(l) == &None {
  return stack[0];
}
let i = &rpn[l];
if i == "!" {
  let num = factorial(&stack[0],1.0);
  stack.remove(0);
  stack.insert(0,num);
  l += 1;
  continue;
}
if i == "s" {
  let num = &stack[0].sqrt();
  stack.remove(0);
  stack.insert(0,*num);
  l += 1;
  continue;
}
if has(["+","-","*","/","^","%"],&i) {
  if &stack.get(1) == &None {
  display_error::error("Kalculater: Error parsing expression: Operator at unexpected place.");
  }
  let nums = [&stack[0], &stack[1]];
  let num = match i.as_str() {
    "+" => nums[1] + nums[0],
    "-" => nums[1] - nums[0],
    "*" => nums[1] * nums[0],
    "/" => nums[1] / nums[0],
    "^" => nums[1].powf(*nums[0]),
    _ => nums[1].rem_euclid(*nums[0])
  };
  stack.remove(0);
  stack.remove(0);
  stack.insert(0,num);
} else {
  if i.chars().last().unwrap() == '!' {
    stack.insert(0,factorial(&parse(&i[0..i.len()-1]),1.0));   
  } else if i.chars().last().unwrap() == 's' {
    let numb = &parse(&i[0..i.len()-1]);
    stack.insert(0,numb.sqrt());
  } else {
  stack.insert(0,parse(i));
  }
}
l += 1;}}