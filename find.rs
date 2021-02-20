#[path = "is_num.rs"]
mod is_num;

pub fn replace(chars: Vec<&str>,toreplacewith: &str, mut eq: Vec<String>,index: usize,lastparen:bool) -> Vec<String> {
if index == 0 {
  eq.insert(0,"".to_string());
}
if eq.get(index+chars.len()-1) == None {
  eq.remove(0);
  return eq.to_vec();
}
let slice = &eq[index..(chars.len()+index)];
let mut ismatch = true;
for i in 0..chars.len() {
  if slice[i] != chars[i] {
    if !(
      chars[i] == "!nu" && (index == 0 || !is_num::parse(&slice[i].to_string()))
    ) {
    if !(chars[i] == "nu" && is_num::parse(&slice[i].to_string())) {
    ismatch = false;
    }
    }
  }
}
if ismatch {
  let replacewith = toreplacewith.chars().map(|x| x.to_string());
  if lastparen {
  eq.splice(index..index+chars.len()-1,replacewith);
  } else {
  eq.splice(index+1..index+chars.len(),replacewith);
  }
  return replace(chars,toreplacewith,eq,index+1,lastparen);
} else {
  return replace(chars,toreplacewith,eq,index+1,lastparen)
}
}