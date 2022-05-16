#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

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

// 1.3.39 Ring buffer. A ring buffer, or circular queue, is a FIFO data structure of a fixed size N. 
// It is useful for transferring data between asynchronous processes or for storing log files. 
// When the buffer is empty, the consumer waits until data is deposited; 
// when the buffer is full, the producer waits to deposit data. 
// Develop an API for a RingBuffer and an implementation that uses an array representation (with circular wrap-around).

struct Ringbuffer<T> where T: std::fmt::Debug {
    ringBuffer: Vec<T>,
    size: usize,
    capacity: usize,
    producerAuxBuffer: Queue<T>,
    dataCountToBeConsumed: usize
}

impl<T> Ringbuffer<T> where T: std::fmt::Debug {
    pub fn new(capacity: usize) -> Self {
        Ringbuffer {
            ringBuffer: Vec::new(),
            size: 0,
            capacity: capacity,
            producerAuxBuffer: Queue::new(),
            dataCountToBeConsumed: 0
        }
    }
    
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn produce(&mut self, item: T) {
        if self.dataCountToBeConsumed > 0 {
            // Check consume requests, if any send direct to be consumed
            // Otherwise store in ringBuffer
            println!("{:?}", item);
            self.dataCountToBeConsumed = self.dataCountToBeConsumed - 1;
        
        // Else if no current consumption requests
        } else {
            if self.isEmpty() {
                // If empty, push onto ringBuffer
                self.ringBuffer.push(item);
                self.size = self.size + 1;
            } else {
                // If there is space remaining
                if self.size < self.capacity {
                    self.ringBuffer.push(item);
                    self.size = self.size + 1;
                // If no space left in ringBuffer, put into the auxillary buffer
                } else {
                    self.producerAuxBuffer.enqueue(item);
                }

            }
        }

    }

    pub fn consume(&mut self) -> Option<T> {
        if self.isEmpty() {
            // Consumer wanted to consume, found nothing, log request to consume
            self.dataCountToBeConsumed = self.dataCountToBeConsumed + 1;
            return None
        }

        // Take first element out, shift all remaining elements to the left
        let item = self.ringBuffer.remove(0);
        self.size = self.size - 1;

        // One slot opened up in ring buffer, so now empty producerAuxBuffer
        if !self.producerAuxBuffer.isEmpty() {
            let item_to_produce = self.producerAuxBuffer.dequeue().unwrap();
            self.produce(item_to_produce);
        }

        Some(item)
    }

    pub fn checkBuffer(&self) -> &Vec<T> {
        &self.ringBuffer
    }
}


fn main () {
    let mut ringbuffer = Ringbuffer::new(4);
    ringbuffer.produce(0);
    ringbuffer.produce(1);
    ringbuffer.produce(2);
    ringbuffer.produce(3);
    ringbuffer.produce(4);
    ringbuffer.produce(5);

    println!("{:?}", ringbuffer.consume());
    println!("{:?}", ringbuffer.consume());

    ringbuffer.produce(6);
    ringbuffer.produce(7);

    println!("{:?}", ringbuffer.checkBuffer());
}

// Lol wtf, too complicated, just use a simple message queue and be done with it