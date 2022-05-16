#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.1 - Show that the number of different triples that can be chosen from N items is precisely 
// N(N-1)(N-2)/6. Hint : Use mathematical induction.

// Base case, N = 3, we know the number of triples is 1
// 3 * (3 - 1) * (3 - 2) / 6 = 3 * 2 * 1 / 6 = 6/6 = 1
// true

// Assume that it is true for N, now prove it is true for N + 1
// If it is true for N, P(N) = N(N-1)(N-2)/6
// Prove that P(N+1) = (N - 1)(N)(N + 1)/6

// P(N+1) = P(N) + "amount of new triples involving N+1"
// = P(N) + "amount of doubles from N items"
// = P(N) + "N choose 2"
// = P(N) + N! / 2! (N-2)!
// = P(N) + N * (N-1) / 2
// = N(N-1)(N-2)/6 + N * (N-1) / 2
// = N(N-1)(N-2)/6 + 3N * (N-1) / 6
// = (N^3 - 3N^2 + 2N +3N^2 - 3N)/6
// = (N^3 - N)/6
// = N(N^2 - 1)/6
// = N(N+1)(N-1)/6 
// QED

fn main() {
}

// Direct proof?
// If you have N items, and want to find the amount of triples
// A triple is a combination, so "N choose 3"
// P(N) = n! / 3! (n -3)! = (n)(n-1)(n-2) / 6