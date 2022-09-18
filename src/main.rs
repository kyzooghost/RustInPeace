#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p558 - dive deeper

use std::hash::{Hash};
mod utils {pub mod LinearProbingHashTable; pub mod RedBlackBST;}
use utils::LinearProbingHashTable::LinearProbingHashTable as ST;
use utils::RedBlackBST::RedBlackBST as RangeST;

pub struct List<T: Copy + Clone + PartialEq + PartialOrd + std::fmt::Debug + Hash> {
    st: ST<T, f32>,
    range_st: RangeST<f32, T>,
    INITIAL_VALUE: f32,
    INTERVAL: f32
}

// index 0 = back, index (size - 1) = front

impl<T: Copy + Clone + PartialEq + PartialOrd + std::fmt::Debug + Hash> List<T> {
    pub fn new() -> Self {
        List {st: ST::new(997), range_st: RangeST::new(), INITIAL_VALUE: 1000000.0, INTERVAL: 1.0}
    }

    pub fn size(&self) -> usize {self.range_st.size()}
    pub fn isEmpty(&self) -> bool {self.range_st.size() == 0}
    pub fn contains(&self, item: &T) -> bool {self.st.contains(item)}

    pub fn addFront(&mut self, item: T) {
        let index: f32;
        if self.contains(&item) {self.delete(self.size() - 1)}
        if self.isEmpty() {
            index = self.INITIAL_VALUE;
        } else {
            index = self.range_st.max().unwrap() + self.INTERVAL;
        }
        self.st.put(item, index);
        self.range_st.put(index, item);
    }

    pub fn addBack(&mut self, item: T) {
        let index: f32;
        if self.contains(&item) {self.delete(0)}
        if self.isEmpty() {index = self.INITIAL_VALUE}
        else {index = self.range_st.min().unwrap() - self.INTERVAL}
        self.st.put(item, index);
        self.range_st.put(index, item);
    }

    pub fn deleteFront(&mut self) {
        if self.isEmpty() {return}
        let index = self.range_st.max().unwrap();
        let item = self.range_st.get(index).unwrap();
        self.range_st.deleteMax();
        self.st.delete(item);
    }

    pub fn deleteBack(&mut self) {
        if self.isEmpty() {return}
        let index = self.range_st.min().unwrap();
        let item = self.range_st.get(index).unwrap();
        self.range_st.deleteMin();
        self.st.delete(item);
    }

    pub fn delete(&mut self, rank: usize) {
        if let Some(index) = self.range_st.select(rank) {
            let item = self.range_st.get(index).unwrap();
            self.range_st.delete(index);
            self.st.delete(item);
        }
    }

    pub fn add(&mut self, item: T, rank: usize) {
        if rank > self.size() {return}
        if rank == 0 {return self.addBack(item)}
        if rank == self.size() {return self.addFront(item)}

        // Find current 'rank' index, and 'rank - 1' index
        let index = self.range_st.select(rank).unwrap();
        let indexMinusOne = self.range_st.select(rank - 1).unwrap();

        // We want to insert at an index between current `rank` and `rank - 1`
        let new_index = 0.5 * (index + indexMinusOne);
        self.range_st.put(new_index, item);
        self.st.put(item, new_index);
    }

    pub fn list(&self) -> Vec<T> {
        let mut vec = Vec::new();
        for i in 0..self.size() {
            // println!("rank - {:?}", i);
            let index = self.range_st.select(i).unwrap();
            let element = self.range_st.get(index).unwrap();
            // println!("index - {:?}", index);
            // println!("element - {:?}", element);
            vec.push(element);
        }
        vec
    }

}

fn main() {
    let mut list = List::new();
    list.addFront(1);
    list.addFront(0);
    list.addBack(10);
    list.addBack(11);
    assert!(list.size() == 4);
    assert!(list.isEmpty() == false);
    assert!(list.list() == vec![11, 10, 1, 0]);
    list.add(9, 2);
    assert!(list.list() == vec![11, 10, 9, 1, 0]);
    list.add(-1, 0);
    assert!(list.list() == vec![-1, 11, 10, 9, 1, 0]);
    list.deleteBack();
    assert!(list.list() == vec![11, 10, 9, 1, 0]);
    list.deleteFront();
    assert!(list.list() == vec![11, 10, 9, 1]);
    list.delete(2);
    assert!(list.list() == vec![11, 10, 1]);
    list.delete(0);
    assert!(list.list() == vec![10, 1]);
    list.delete(5);
    assert!(list.list() == vec![10, 1]);
    list.deleteFront();
    list.deleteBack();
    assert!(list.list() == vec![]);
}