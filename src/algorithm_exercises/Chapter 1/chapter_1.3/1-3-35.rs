#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.3.35 - Random queue. A random queue stores a collection of items
// 

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

    let mut hand1 = Vec::new();
    let mut hand2 = Vec::new();
    let mut hand3 = Vec::new();
    let mut hand4 = Vec::new();

    for _ in 0..13 {
        hand1.push(bag.dequeue());
        hand2.push(bag.dequeue());
        hand3.push(bag.dequeue());
        hand4.push(bag.dequeue());
    }

    println!("{:?}", hand1);
    println!("-------------");
    println!("{:?}", hand2);
    println!("-------------");
    println!("{:?}", hand3);
    println!("-------------");
    println!("{:?}", hand4);
    println!("-------------");

    assert_eq!(bag.isEmpty(), true);
}