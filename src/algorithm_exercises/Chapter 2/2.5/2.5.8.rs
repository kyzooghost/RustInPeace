#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p366 Exercises

// 2.5.8

// Write a program Frequency that reads strings from standard input and 
// prints the number of times each string occurs, in descending order of frequency.

// Can use an indexed maximum priority queue here
// Maintain a hashmap of index => string
// For each new string, check hashmap
// => If index exists, then increment value by 1
// => Else if index doesn't exist, insert 1 into the priority queue, and add hashmap index => string entry

// To get the descending order frequency list, continually deleteMax() and use hashmap to map index to string

fn main() {
}