#[path = "is_num.rs"]
mod is_num;
#[path = "get_num.rs"]
mod get_num;
#[path = "display_error.rs"]
mod display_error;
// Shunting yard algorithm
pub fn convert(equation:Vec<String>,mut rpn: Vec<String>, mut op_stack: Vec<(u16,String,bool)>,index:usize,previous:String) -> (Vec<String>,Vec<(u16,String,bool)>) {
  // Break recursion
  if equation.get(index) == None {
    return (rpn, op_stack);
  }
  let elemnum = get_num::find(equation[index..].to_vec(),previous);
  if is_num::parse(&elemnum) {
    // Regular number
    rpn.push(elemnum.clone());
  } else {
    // Operator
    let mut elemnum_precedence: u16 = 2;
    let mut lefttoright = false;
    if elemnum == "*" || elemnum == "/" || elemnum == "%" {
      elemnum_precedence = 3;
    } else if elemnum == "^" || elemnum == "s" || elemnum == "!"  {
      elemnum_precedence = 4;
      lefttoright = true;
    }
    if elemnum == "(" {
    op_stack.insert(0,(elemnum_precedence,elemnum.clone(),false));
    } else if elemnum == ")" {
      loop {
        if op_stack.len() == 0 {
          display_error::error("Kalculator: Unbalanced parenthesis");
        }
        if op_stack[0].1 != "(" {
          let num = &op_stack[0].1;
          rpn.push(num.to_string());
          op_stack.remove(0);
        } else {
          op_stack.remove(0);
          break;
        }
      }
    } else {
    while (&op_stack.len() > &0) && ((&op_stack[0].0 > &elemnum_precedence) || (&op_stack[0].0 == &elemnum_precedence && op_stack[0].2)) && (&op_stack[0].1 != "(") {
      rpn.push((&*op_stack[0].1).to_string());
      op_stack.remove(0);
    }
    op_stack.insert(0,(elemnum_precedence,elemnum.clone(),lefttoright));
    }
  }
  return convert(equation[elemnum.len()-1..].to_vec(),rpn,op_stack,index+1,elemnum);
}