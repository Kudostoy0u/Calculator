pub fn parse(possible_string: &String) -> bool {
  let mut isnum = true;
  let mut period_occured = false;
  let mut nums = possible_string.split("").collect::<Vec<_>>();
  nums.pop();
  nums.remove(0);
  if possible_string.len() == 0 {
    return false;
  }
  for i in 0..possible_string.len() {
    let iscurrentnum = match nums[i] {
      "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
      "." => {
        if !period_occured {
          period_occured = true;
          if i > 0 {
          return true;
          } else {
            return false;
          }
        } else {
          return false;
        }
      },
      "-" | "+" => {
        if i == 0 && (nums.get(i+1) != None) && parse(
          &nums[i+1].to_string()) {
          return true;
        } else {
          return false;
        }
      },
      "!" | "s" => {
        if i == possible_string.len()-1 && possible_string.len() != 1 {
          return true;
        } else {
          return false;
        }
      },
      _ => false
    };
    if !iscurrentnum {
      isnum = false;
      break;
    }
  }
  return isnum;
}