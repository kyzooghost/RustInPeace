#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.9

// Give a formula to predict the running time of a program for a problem of size N when doubling experiments 
// have shown that the doubling factor is 2^b and the running time for problems of size N0 is T.

// Number of doublings from N0 to N
// 1 to N, how many doublings?
// So divide N/N0
// log 2 N/N0

// Each doubling multiples running time by 2^b time

// So running time of size N = T * (doubling factor) ^ (# of doublings)
// = T * 2 ^ b ^ ( log 2 N/N0 )
// = T * 2 ^ (b log 2 N/N0)
// = T * (N/N0) ^ b

fn main() {
}