#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
struct Queue<T> where T: std::fmt::Debug {
    size: usize,
    vec: Vec<T>
}

impl<T> Queue<T> where T: std::fmt::Debug {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Queue {size: 0, vec: Vec::new()}
    }

    pub fn enqueue(&mut self, elem: T) {
        self.vec.push(elem);
        self.size = self.size + 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;
            Some(self.vec.remove(0))
        }
    }
}

// 1.3.41 Copy a queue

fn main () {
    let mut queue1 = Queue::new();
    queue1.enqueue(0);
    queue1.enqueue(1);
    queue1.enqueue(2);
    queue1.enqueue(3);
    queue1.enqueue(4);

    println!("{:?}", queue1);

    let mut buffer = Vec::new();

    let size = queue1.size;

    for _ in 0..size {
        buffer.push(queue1.dequeue().unwrap());
    }

    let mut buffer_clone = buffer.clone();
    let mut queue2 = Queue::new();

    for _ in 0..size {
        queue1.enqueue(buffer.remove(0));
        queue2.enqueue(buffer_clone.remove(0));
    }

    println!("{:?}", queue1);
    println!("{:?}", queue2);

}