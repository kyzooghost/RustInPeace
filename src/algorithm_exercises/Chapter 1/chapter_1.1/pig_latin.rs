fn main() {
  let s1 = String::from("first");
  println!("{}", convert_to_pig_latin(&s1));
}

fn convert_to_pig_latin(_string: &str) -> String {
  use std::str::from_utf8;
  
  let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
  let first_letter =  _string.as_bytes()[0] as char;

  // Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
  for letter in vowels {
    if first_letter == letter {
      let return_string = format!("{}-hay", _string);
      return return_string
    }
  }

  // Otherwise the first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” 

  let remaining_letters = from_utf8(&_string.as_bytes()[1..]).unwrap();

  let return_string = format!("{}-{}ay", remaining_letters, first_letter);
  
  return return_string
}