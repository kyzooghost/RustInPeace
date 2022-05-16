// A string s is a circular rotation of a string t if it matches when the characters
// are circularly shifted by any number of positions; e.g., ACTGACG is a circular shift of
// TGACGAC, and vice versa. Detecting this condition is important in the study of genomic
// sequences. Write a program that checks whether two given strings s and t are circular shifts of one another. Hint : The solution is a one-liner with indexOf(), length(), and
// string concatenation.

// 1.2.6

fn main() { 
  let s = String::from("ACTGACG");
  let t = String::from("TGACGAC");
  let status = is_circular_rotation(s, t);
  println!{"{}", status};
}

fn is_circular_rotation(string_1: String, string_2: String) -> bool {
  let copy_string_1 = string_1.clone();
  let double_string_1 = copy_string_1 + &string_1;
  println!{"{}", double_string_1};
  if string_1.len() == string_2.len() && double_string_1.contains(&string_2) {return true}
  false
}