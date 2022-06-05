pub struct MaxPQ<T> {
    heap: Vec<Option<T>>,
    size: usize
}

pub struct MinPQ<T> {
    heap: Vec<Option<T>>,
    size: usize
}

impl<T: Copy + Ord + std::fmt::Debug> MaxPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        MaxPQ { size: 0, heap: vec![None] }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn max(&self) -> Option<T> {
        if self.size == 0 {return None}
        self.heap[1]
    }

    pub fn insert(&mut self, element: T) {
        self.size += 1;
        self.heap.push(Some(element));
        self.swim(self.size);
    }

    pub fn delMax(&mut self) -> Option<T> {
        if self.size == 0 {return None}

        self.heap.swap(1, self.size);
        let max = self.heap.pop().unwrap();
        self.size -= 1;
        self.sink(1);
        max
    }

    fn swim(&mut self, mut index: usize) {
        while index > 1 && self.heap[index / 2] < self.heap[index] {
            self.heap.swap(index, index / 2);
            index = index / 2;
        }
    }

    fn sink(&mut self, mut index: usize) {
        while 2*index <= self.size {
            let mut j = 2 * index;
            if j < self.size && self.heap[j] < self.heap[j + 1] {j += 1}
            if self.heap[index] >= self.heap[j] {break}
            self.heap.swap(index, j);
            index = j;
        }   
    }
}

impl<T: Copy + Ord + std::fmt::Debug> MinPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        MinPQ { size: 0, heap: vec![None] }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn min(&self) -> Option<T> {
        if self.size == 0 {return None}
        self.heap[1]
    }

    pub fn insert(&mut self, element: T) {
        self.size += 1;
        self.heap.push(Some(element));
        self.swim(self.size);
    }

    pub fn delMin(&mut self) -> Option<T> {
        if self.size == 0 {return None}

        self.heap.swap(1, self.size);
        let max = self.heap.pop().unwrap();
        self.size -= 1;
        self.sink(1);
        max
    }

    fn swim(&mut self, mut index: usize) {
        while index > 1 && self.heap[index / 2] > self.heap[index] {
            self.heap.swap(index, index / 2);
            index = index / 2;
        }
    }

    fn sink(&mut self, mut index: usize) {
        while 2*index <= self.size {
            let mut j = 2 * index;
            if j < self.size && self.heap[j] > self.heap[j + 1] {j += 1}
            if self.heap[index] <= self.heap[j] {break}
            self.heap.swap(index, j);
            index = j;
        }   
    }
}
