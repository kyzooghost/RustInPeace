#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.33

// Memory requirements on a 32-bit machine. 

// Give the memory requirements for Integer, Date, Counter, int[], double[], double[][], String, Node, 
// and Stack (linked-list representation) for a 32-bit machine. 

// Assume that references are 4 bytes, 
// object overhead is 8 bytes, 
// and padding is to a multiple of 4 bytes.


// Integer
// 4 bytes for the primitive field + 8 bytes for overhead = 12 bytes

// Date
// 8 bytes for primitive long + 8 bytes overhead = 16 bytes

// Counter
// Is just an integer? 12 bytes
// Object overhead + int + String ref? = 16 bytes

// int[]
// 4 byte reference + 8 byte object overhead + 4N bytes for each int element = 12 + 4N bytes

// double[N] = 12 bytes + 8N bytes

// double[M][N] = 12 + 16*M + 8*MN
// 8 bytes for each individual double element, MN of these elements
// For object representing each [N] array, 8 object overhead + 4 int count + 4 reference = 16, M of these

// String object itself = overhead (8) + ref to char[] (4) + hash int (4) = 16
// char [] = 12 bytes (overhead + int for length) + 2 bytes for each element
// 28 + 2N

// Node
// Overhead (8) + Item ref (4) + Forward ref (4) + Backward ref (4) = 20

// Linked List
// Linked List Object itself - overhead (8) + ref to first node (4) + size int (4)
// For each Node, 20N
// For each int, 12N
// 16 + 32N

fn main() {
}