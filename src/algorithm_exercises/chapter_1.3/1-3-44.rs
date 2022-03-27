#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
struct Stack<T> {
    size: usize,
    vec: Vec<T>
}

impl<T> Stack<T> {
    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn new() -> Self {
        Stack {size: 0, vec: Vec::new()}
    }

    pub fn push(&mut self, elem: T) {
        self.vec.push(elem);
        self.size = self.size + 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {None}
        else {
            self.size = self.size - 1;
            self.vec.pop()
        }
    }
}

// 1.3.44 Text editor buffer. 
// Develop a data type for a buffer in a text editor that implements the following API:

#[derive(Debug)]
struct Buffer<char> {
    chars_before_cursor: Stack<char>,
    chars_after_cursor: Stack<char>,
    size: usize,
}

impl Buffer<char> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn cursor_position(&self) -> usize {
        self.chars_before_cursor.size
    }

    pub fn new() -> Self {
        Buffer {
            chars_before_cursor: Stack::new(),
            chars_after_cursor: Stack::new(),
            size: 0
        }
    }

    pub fn insert(&mut self, c: char) {
        self.size = self.size + 1;
        self.chars_before_cursor.push(c);
    }

    pub fn delete(&mut self) -> Option<char> {
        self.size = self.size - 1;
        self.chars_before_cursor.pop()
    }

    pub fn left(&mut self, mut k: usize) {
        if k > self.chars_before_cursor.size {
            k = self.chars_before_cursor.size;
        }
        
        for _ in 0..k {
            self.chars_after_cursor.push ( self.chars_before_cursor.pop().unwrap() );
        }
    }

    pub fn right(&mut self, mut k: usize) {
        if k > self.chars_after_cursor.size {
            k = self.chars_after_cursor.size;
        }

        for _ in 0..k {
            self.chars_before_cursor.push ( self.chars_after_cursor.pop().unwrap() );
        }
    }
}

fn main() {
    let mut buffer = Buffer::new();
    buffer.insert('a');
    buffer.insert('b');
    buffer.insert('c');
    buffer.insert('d');
    buffer.insert('e');
    buffer.left(3);
    println!("{:?}", buffer.delete());
    println!("{:?}", buffer.delete());
}
