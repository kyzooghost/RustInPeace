#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.34

// Hot or cold. 
// Your goal is to guess a secret integer between 1 and N. 
// You repeatedly guess integers between 1 and N. 
// After each guess you learn if your guess equals the secret integer (and the game stops). 

// Otherwise, you learn if the guess is hotter (closer to) or colder (farther from) 
// the secret number than your previous guess. 
// Design an algorithm that finds the secret number in at most ~2 lg N guesses. 

// Then design an algorithm that finds the secret number in at most ~ 1 lg N guesses.

// So you don't get a 'hotter' or 'colder' indicator until you have done your first guess
// Place your first guess in the middle
// Then place second guess in the middle of the first half

// If hotter, you again place in middle of first half, within the first half
// If colder, you put in the middle of the second half, then you place in the middle of the first half, within that second half
// Repeat until you get the secret integer

fn main() {
}