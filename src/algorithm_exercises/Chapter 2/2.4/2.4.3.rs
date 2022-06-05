#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p329 Exercises

// 2.4.3

// Priority queue implementation that support insert and remove the maximum, for unordered array and array

// Unordered array - insert is O(1), delMax is O(N)
// Ordered array - insert is O(N), delMax is O(1)
// Unordered linked list - insert is O(1), delMax is O(N)
// Ordered linked list - insert is O(N), delMax is O(1)

pub struct MaxPQ<T> {
    array: Vec<T>,
    size: usize
}

impl<T: Copy + Ord + std::fmt::Debug> MaxPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        MaxPQ { size: 0, array: Vec::new() }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn insert(&mut self, element: T) {
        self.array.push(element);
        self.size += 1;
    }

    pub fn max(&self) -> Option<&T> {
        if self.size == 0 {return None}
        let mut max = &self.array[0];
        for element in self.array.iter() {if max > element {max = element;}}
        Some(max)
    }

    pub fn delMax(&mut self) -> Option<T> {
        if self.size == 0 {return None}

        let mut max = &self.array[0];
        let mut max_index = 0;
        for (index, element) in self.array.iter().enumerate() {
            if max > element {
                max = element;
                max_index = index;
            }
        }

        self.size -= 1;
        Some(self.array.remove(max_index))
    }
}

fn main() {
    let mut pq = MaxPQ::new();
    pq.insert("E");
}
