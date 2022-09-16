#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p522

// 3.5.24 - Non-overlapping interval search
// Non-overlapping interval search. Given a list of non-overlapping intervals of items, write a function that takes an item as argument and determines in which, if any, interval that item lies. For example, if the items are integers and the intervals are 1643-2033, 5532-7643, 8999-10332, 5666653-5669321, then the query point 9122 lies in the third interval and 8122 lies in no interval.

// Represent each interval using a red-black tree, min and max function define interval bounds
// Store intervals as array of intervals

mod utils {pub mod RedBlackTree;}
use utils::RedBlackTree::RedBlackBST as RangeST;

pub struct Interval {
    st: RangeST<usize, usize>
}

impl Interval {
    pub fn new(min: usize, max: usize) -> Self {
        assert!(max > min, "max must > min");
        let mut st = RangeST::new();
        for i in min..max+1{st.put(i,i);}
        Interval{st: st}
    }

    pub fn min(&self) -> usize {self.st.min().unwrap()}
    pub fn max(&self) -> usize {self.st.min().unwrap()}

    pub fn contains(&self, number: usize) -> bool {
        match self.st.get(number) {
            None => {return false},
            Some(_) => {return true}
        }
    }
}

pub struct IntervalCollection {
    collection: Vec<Interval>
}

impl IntervalCollection {
    pub fn new() -> Self {
        IntervalCollection{collection: Vec::new()}
    }

    pub fn addInterval(&mut self, interval: Interval) {
        self.collection.push(interval)
    }

    pub fn contains(&self, number: usize) -> bool {
        for i in 0..self.collection.len() {
            if self.collection[i].contains(number) {return true}
        }

        false
    }
}

fn main() {
    let interval1 = Interval::new(1643, 2033);
    let interval2 = Interval::new(5532, 7643);
    let interval3 = Interval::new(8999, 10332);
    let interval4 = Interval::new(5666653, 5669321);
    let mut set = IntervalCollection::new();
    set.addInterval(interval1);
    set.addInterval(interval2);
    set.addInterval(interval3);
    set.addInterval(interval4);

    println!("{:?}", set.contains(9122));
    println!("{:?}", set.contains(8122));
}