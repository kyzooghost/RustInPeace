#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.3.37 Josephus problem. 
// In the Josephus problem from antiquity, 
// N people are in dire straits and agree to the following strategy to reduce the population. 
// They arrange themselves in a circle (at positions numbered from 0 to N–1) and proceed around the circle, 
// eliminating every Mth person until only one person is left. 
// Legend has it that Josephus figured out where to sit to avoid being eliminated. 
// Write a Queue client Josephus that takes N and M from the command line 
// and prints out the order in which people are eliminated (and thus would show Josephus where to sit in the circle).



struct Queue<T> where T: std::fmt::Debug {
    size: usize,
    queue: Vec<T>
}


impl<T> Queue<T> where T: std::fmt::Debug {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Queue {size: 0, queue: Vec::new()}
    }

    pub fn enqueue(&mut self, elem: T) {
        self.queue.push(elem);
        self.size = self.size + 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;
            Some(self.queue.remove(0))
        }
    }
}

fn main () {
    println!("{:?}", josephus(7, 2));
}

fn josephus(n: usize, m: usize) -> usize {
    
    let mut vec = Vec::new();
    for i in 0..n {vec.push(i);}

    let mut queue = Queue::new();

    for i in 0..n-1 {
        queue.enqueue( vec.remove(m-1) );

        // shuffle the vector, anything to the left of this index needs to be pushed to the end of the vec
        for j in 0..m-1 {
            let removed_element = vec.remove(0);
            vec.push(removed_element)
        }
    }

    assert!(vec.len() == 1, "did not go through whole circle");

    for i in 0..n-1 {
        println!("{:?}", queue.dequeue().unwrap());
    }

    vec.pop().unwrap()
}