#![allow(non_snake_case)]

pub mod queue {
  #[derive(Debug)]
  pub struct ResizingArrayQueueOfStrings {
    size: usize, // size
    pub a: Vec<String>, // stack entries
    front_index: usize,
    rear_index: usize
  }

  impl ResizingArrayQueueOfStrings {
    fn resize(&mut self, new_size: usize) {
      let mut temp = vec![String::new(); new_size];
      
      if self.front_index == self.rear_index {temp[0] = self.a[self.front_index].clone();}
      else {
        for i in self.front_index..self.size {
          temp[i] = self.a[i].clone();
        }
      }

      self.front_index = 0;

      if self.rear_index == 0 {self.rear_index = 0;}
      else {self.rear_index = self.size - 1;}

      self.a = temp;
    }

    pub fn size(&self) -> usize {
      self.size
    }

    pub fn isEmpty(&self) -> bool {
      self.size == 0
    }

    pub fn enqueue(&mut self, item:String) {
      if self.a.len() == self.size {self.resize(self.size * 2);}

      if self.size == 0 {
        self.a[self.size] = item;
        self.size = self.size + 1;
      } else {
        self.rear_index = self.rear_index + 1;
        self.a[self.rear_index] = item;
        self.size = self.size + 1;
      }
    }

    pub fn dequeue(&mut self) -> String {
      assert!(self.size > 0, "nothing to dequeue");

      let dequeued_element: String = self.a[self.front_index].clone();
      self.a[self.front_index] = "".to_string();
      self.size = self.size - 1;

      if self.front_index < self.rear_index {self.front_index = self.front_index + 1;}
      else {assert!(self.front_index <= self.rear_index, "dequeue: front_index jumped in front of rear_index");}

      println!("{}", self.front_index);
      if self.size <= self.a.len() / 4 {self.resize(self.a.len() / 2)}

      dequeued_element
    }

    pub fn new() -> Self {
      ResizingArrayQueueOfStrings { size: 0, a: vec![String::new(); 1], front_index: 0, rear_index: 0 }
    }
  }
  
}

// 1.3.15
// Write a Queue client that takes a command-line argument k and prints the kth from the last string found 
// on standard input (assuming that standard input has k or more strings). 

// Re-interpret: write a function that takes two parameters, k and a sentence,
// and returns the kth from the last string in the sentence

fn main() {
  let sentence = String::from("The balloons floated away along with all my hopes and dreams.");
  let k:usize = 4;
  assert!(printWord(sentence, k) == "my", "incorrect printWord() implementation");
  println!("success!");
}

fn printWord(sentence: String, k: usize) -> String {
  // Take your sentence, from beginning insert into queue
  // At the kth instance, dequeue? Isn't a stack better for this?

  let mut count:usize = 0;
  let mut returned_word = String::from("");
  let mut word_queue = queue::ResizingArrayQueueOfStrings::new();
  let words = sentence.split_whitespace().collect::<Vec<&str>>();

  for word in words {
    word_queue.enqueue(word.to_string());
  }

  for word in word_queue.a.clone() {
    count = count + 1;

    if count == word_queue.size() - (k - 1) {
      returned_word = word;
      break;
    }

  }

  returned_word
}