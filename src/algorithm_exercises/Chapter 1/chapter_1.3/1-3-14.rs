#[allow(non_snake_case)]

pub mod queue {
  #[derive(Debug)]
  pub struct ResizingArrayQueueOfStrings {
    size: usize, // size
    a: Vec<String>, // stack entries
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

// 1.3.14
// Develop a class ResizingArrayQueueOfStrings that implements the queue abstraction with a fixed-size array, 
// and then extend your implementation to use array resizing to remove the size restriction.

fn main() {
  let mut a = queue::ResizingArrayQueueOfStrings::new();
  a.enqueue("1".to_string());
  println!("{:?}", a);
  a.enqueue("2".to_string());
  println!("{:?}", a);
  a.enqueue("3".to_string());
  println!("{:?}", a);
  a.enqueue("Full".to_string());
  println!("{:?}", a);
  a.dequeue();
  println!("{:?}", a);
  a.dequeue();
  println!("{:?}", a);
  a.dequeue();
  println!("{:?}", a);
  a.dequeue();
  println!("{:?}", a);
}