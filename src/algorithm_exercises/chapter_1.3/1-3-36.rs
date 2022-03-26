#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.3.36 Random iterator. Write an iterator for RandomQueue<Item> from the previous exercise 
// that returns the items in random order.


use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

struct RandomQueue<T> where T: std::fmt::Debug {
    size: usize,
    queue: Vec<T>
}

pub struct IntoIter<T>(RandomQueue<T>) where T: std::fmt::Debug;

impl<T> RandomQueue<T> where T: std::fmt::Debug {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        RandomQueue {size: 0, queue: Vec::new()}
    }

    pub fn enqueue(&mut self, elem: T) {
        self.queue.push(elem);
        self.queue.shuffle(&mut thread_rng());
        self.size = self.size + 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.queue.shuffle(&mut thread_rng());
            self.size = self.size - 1;
            self.queue.pop()
        }
    }

    pub fn sample(&self) -> Option<&T> {
        if self.size == 0 {None}
        else {
            Some(&self.queue[rand::thread_rng().gen_range(0..self.size)])
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> where T: std::fmt::Debug{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.queue.shuffle(&mut thread_rng());
        self.0.queue.pop()
    }
}

fn main () {
    let mut bag = RandomQueue::new();

    for _ in 0..4 {
        bag.enqueue(Card::Ace);
        bag.enqueue(Card::Two);
        bag.enqueue(Card::Three);
        bag.enqueue(Card::Four);
        bag.enqueue(Card::Five);
        bag.enqueue(Card::Six);
        bag.enqueue(Card::Seven);
        bag.enqueue(Card::Eight);
        bag.enqueue(Card::Nine);
        bag.enqueue(Card::Ten);
        bag.enqueue(Card::Jack);
        bag.enqueue(Card::Queen);
        bag.enqueue(Card::King);
    }

    let mut iter = bag.into_iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}