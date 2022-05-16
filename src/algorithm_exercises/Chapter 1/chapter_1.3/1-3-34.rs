#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.3.34 Random bag. A random bag stores a collection of items

use rand::thread_rng;
use rand::seq::SliceRandom;

struct RandomBag<T> {
    size: usize,
    bag: Vec<T>
}

impl<T> RandomBag<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        RandomBag {size: 0, bag: Vec::new()}
    }

    pub fn add(&mut self, elem: T) {
        self.bag.push(elem);
        self.bag.shuffle(&mut thread_rng());
        self.size = self.size + 1;
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;
            self.bag.pop()
        }
    }
}

fn main () {
    let mut bag = RandomBag::new();
    bag.add(1);
    println!("{:?}", bag.isEmpty());
    println!("{:?}", bag.remove());
    println!("{:?}", bag.remove());
}