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

// 1.3.43 Listing files 
// A folder is a list of files and folders. 
// Write a program that takes the name of a folder as a command-line argument 
// and prints out all of the files contained in that folder, 
// with the contents of each folder recursively listed (indented) under that folder’s name. 
// Hint : Use a queue, and see java.io.File.

use std::{io, fs, path::Path};

fn main() {
    read_folders_recursively(".", 0);
}

// Read current directory and print
// If is_dir() true, recursive call
// For each recursive stack, indent the output
fn read_folders_recursively<P: AsRef<Path>>(input_path: P, level: usize) -> io::Result<()> {

    // fs::read_dir => Take a path, and return Result<ReadDir<path>>
    // ReadDir is an iterator over DirEntry
    let current_dir = fs::read_dir(input_path)?;

    let mut file_queue = Queue::new();
    let mut temp_queue = Queue::new();

    // use a queue to sort the current_dir, non-folders first
    for item in current_dir {
        let path = item?.path();

        if !path.is_dir() {file_queue.enqueue(path)}
        else {temp_queue.enqueue(path)}
    }

    while !temp_queue.isEmpty() {file_queue.enqueue( temp_queue.dequeue().unwrap() )}

    // display file, tail-recursive if folder
    // parse path name 
    while !file_queue.isEmpty() {
        let raw_path = file_queue.dequeue().unwrap();

        let mut parsed_path: Vec<&str> = raw_path.to_str().unwrap().split("/").collect();
        let parsed_path = parsed_path.last().unwrap().to_owned().to_string();
        
        if raw_path.is_dir() {
            println!("{:indent$}{:?}", "", String::from("/") + &parsed_path, indent=level*5);
        } else {
            println!("{:indent$}{:?}", "", &parsed_path, indent=level*5);            
        }


        if raw_path.is_dir() {
            read_folders_recursively(raw_path, level + 1);
        }
    }

    Ok(())
}
