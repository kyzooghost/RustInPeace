#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p480 Exercises

// 3.4.1

// Insert EASYQUESTION into an initially empty table of M = 5 lists, using separate chaining
// Use hash function 11 k % M to transform the kth letter into a table index

// 0 E Y T O
// 1 A U
// 2 Q
// 3
// 4 S I N

fn main() {
    let vec = vec![
        "E",
        "A",
        "S",
        "Y",
        "Q",
        "U",
        "E",
        "S",
        "T",
        "I",
        "O",
        "N",
    ];

    for letter in vec {
        println!("{:?}", hash_letter(letter));
    }

}

fn letter_to_number(letter: &str) -> usize {
    if letter == "A" {return 1}
    else if letter == "B" {return 2}
    else if letter == "C" {return 3}
    else if letter == "D" {return 4}
    else if letter == "E" {return 5}
    else if letter == "F" {return 6}
    else if letter == "G" {return 7}
    else if letter == "H" {return 8}
    else if letter == "I" {return 9}
    else if letter == "J" {return 10}
    else if letter == "K" {return 11}
    else if letter == "L" {return 12}
    else if letter == "M" {return 13}
    else if letter == "N" {return 14}
    else if letter == "O" {return 15}
    else if letter == "P" {return 16}
    else if letter == "Q" {return 17}
    else if letter == "R" {return 18}
    else if letter == "S" {return 19}
    else if letter == "T" {return 20}
    else if letter == "U" {return 21}
    else if letter == "V" {return 22}
    else if letter == "X" {return 23}
    else if letter == "W" {return 24}
    else if letter == "Y" {return 25}
    else if letter == "Z" {return 26}
    0
}

fn hash_letter(letter: &str) -> usize {
    ( 11 * letter_to_number(letter) ) % 5
}