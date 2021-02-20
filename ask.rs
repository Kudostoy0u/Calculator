#[path = "get_input.rs"]
mod get_input;
#[path = "find.rs"]
mod find;
#[path = "split_by_delimeters.rs"]
mod split_by_delimeters;
#[path = "solution_of.rs"]
mod solution_of;
#[path = "evaluate.rs"]
mod evaluate;
fn remove_trailing_zeros(num: String) -> String {
  let mut trunc = 0;
  for i in num.chars().rev() {
    if i == '0' {
      trunc += 1;
    } else { break; }}
  return num[0..num.len()-trunc].to_string()
}
pub fn equation() {
  print!("\x1B[2J\x1B[1;1H");
  let mut infix = get_input::read("Enter expression: ").replace("**","^").replace("x","*").replace(")(",")*(").replace("pi","3.141592653589793238462643383279").
  chars().filter(|x| x != &' ').map(|x| x.to_string()).collect::<Vec<_>>();
  infix.pop();
  infix = find::replace(vec!["!nu","."],"0.",infix,0,false);
  infix = find::replace(vec!["nu","("],"*(",infix,0,false);
  infix = find::replace(vec![")","nu"],")*",infix,0,true);
  if infix.contains(&">".to_string()) || infix.contains(&"=".to_string()) || infix.contains(&"<".to_string()) {
    let split_expr = split_by_delimeters::split(vec!["<=",">=","<",">","="],infix);
    let mut tokens = Vec::new();
    for i in split_expr {
      if !i.1 {
        tokens.push((solution_of::eq(i.0).to_string(),false));
      } else {
        tokens.push((i.0.join(""),true));
      }
    }
    let answ = evaluate::tok(tokens,1,true);
    if answ {
    println!("\x1b[1;32mtrue\x1b[0m");
    } else {
    println!("\x1b[1;31mfalse\x1b[0m")
    }
  } else {
  let sol = solution_of::eq(infix);
  let mut solution = sol.clone().to_string();
  let solution_as_int = solution.to_string().parse::<isize>();
  if solution_as_int.is_ok() { solution = solution_as_int.unwrap().to_string() }
  if sol > 100000000000.0 {
    solution = format!("{:e}",sol);
  }
  let last = solution[0..solution.len()-1].chars().last();
  if last != None && last.unwrap() == '0' && solution.contains(&".") {
    println!("\x1b[0;32m{}\x1b[0m",remove_trailing_zeros(solution[0..solution.len()-1].to_string()));
  } else {
    println!("\x1b[0;32m{}\x1b[0m",solution);
  }

 }
  get_input::pause();
  equation();
}