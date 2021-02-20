use std::str;
fn priority_split(first: &String, delims: &Vec<&str>,index:usize,mut best: (usize,String)) -> (usize,String) {
if first.split("").collect::<Vec<_>>().get(index) == None || delims.get(index) == None {
  return best;
}
let current = delims[index];
let iterates = first.chars().map(|x| x.to_string()).collect::<Vec<_>>();
let mut idx = 0;
for i in 0..iterates.len() {
  if iterates.get(i+current.len()) != None && iterates[i..i+current.len()].join("") == current {
  if !(current == "=" && (iterates[i-1] == ">" || iterates[i-1] == "<")) {
    if current == ">=" || current == "<=" {
    idx = i+1;
    } else {
      idx = i;
    }
  }
  }
}
if idx != 0 {
  if idx > best.0 {
    best = (idx,current.to_string());
  }
}
return priority_split(first,delims,index+1,best)
}
pub fn split(delims: Vec<&str>,eq: Vec<String>) -> Vec<(Vec<String>,bool)> {
let mut section = Vec::new();
section.push((eq,false));
loop {
let first = section[0].0.join("");
let spliton = priority_split(&first,&delims,0,(0,"".to_string()));
if spliton.1 != "".to_string() {
  let first_sec = &first[..spliton.0-spliton.1.len()+1].chars().map(|x| x.to_string()).collect::<Vec<_>>();
  let second_sec = &first[spliton.0+1..].chars().map(|x| x.to_string()).collect::<Vec<_>>();
  section.remove(0);
  let op = spliton.1.chars().map(|x| x.to_string()).collect::<Vec<_>>();
  section.insert(0,(first_sec.clone(),false));
  section.insert(1,(op,true));
  section.insert(2,(second_sec.clone(),false)); 
} else {
  break;
}
}
return section;
}