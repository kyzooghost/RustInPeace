#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.6 

// a. int sum = 0;
// for (int n = N; n > 0; n /= 2)
//           for(int i = 0; i < n; i++)
//              sum++;

// First iteration - run N times
// Second iteration - run N/2 times
// Third iteration - run N/4 times
// Formula for infinite geometric series = 1 / (1 - 0.5) = 2
// So converge to 2N over time
// ~2N => O(N)


// b. int sum = 0;
// for (int i = 1 i < N; i *= 2)
//             for (int j = 0; j < i; j++)
//                 sum++;

// Say N = 5
// i = 1 -> 1 inner loop
// i = 2 -> 2 inner loop
// i = 4 -> 4 inner loop
// So will run 1 + 2 + 4 + ... times
// But the numbers of times is limited by N
// Number of times = floor(log 2 N) + 1
// Finite geometric series
// a = 1, r = 2, n = log 2 N
// sum = 1(1 - 2^ log 2 N) / -1
// = 1 - N / - 1
// = N
// ~N = O(n)


// c. int sum = 0;
// for (int i = 1 i < N; i *= 2)
//            for (int j = 0; j < N; j++)
//                sum++;

// Inner and outer loops independent of each other
// Outer loop - ~log 2 N + 1
// Inner loop - N
// ~N log N


fn main() {
}
