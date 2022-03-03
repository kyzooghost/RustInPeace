#![allow(non_snake_case)]

// public Date(String date)
// {
//    String[] fields = date.split("/");
//    month = Integer.parseInt(fields[0]);
//    day   = Integer.parseInt(fields[1]);
//    year  = Integer.parseInt(fields[2]);
// }

pub mod Date {
  #[derive(Debug)]
  pub struct Date {
    day: i32,
    month: i32,
    year: i32
  }

  impl Date {
    pub fn new(string: String) -> Self {
      let numbers = string.split("/").collect::<Vec<&str>>();
      assert!(numbers.len() == 3, "not date format");
      Date {
        day: numbers[0].parse::<i32>().unwrap(), 
        month: numbers[1].parse::<i32>().unwrap(), 
        year: numbers[2].parse::<i32>().unwrap()
      } 
    }
  }
  
}

pub mod queue {
  #[derive(Debug)]
  pub struct ResizingArrayQueueOfStrings {
    size: usize, // size
    pub a: Vec<String>, // stack entries
    pub front_index: usize,
    pub rear_index: usize
  }

  impl ResizingArrayQueueOfStrings {
    fn resize(&mut self, new_size: usize) {
      let mut temp = vec![String::new(); new_size];
      
      if self.front_index == self.rear_index {temp[0] = self.a[self.front_index].clone();}
      else {
        let mut count:usize = 0;

        for i in 0..self.size {
          temp[i] = self.a[self.front_index + count].clone();         
          count = count + 1;
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

      if self.size <= self.a.len() / 4 {self.resize(self.a.len() / 2)}

      dequeued_element
    }

    pub fn new() -> Self {
      ResizingArrayQueueOfStrings { size: 0, a: vec![String::new(); 1], front_index: 0, rear_index: 0 }
    }
  }
}

// 1.3.16
// Using readInts() on page 126 as a model, write a static method readDates() for Date 
// that reads dates from standard input in the format specified in the table on page 119 
// and returns an array containing them.

fn main() {
  let date1 = String::from("17/11/1945");
  let date2 = String::from("08/12/1865");
  let date3 = String::from("11/08/1911");
  let date4 = String::from("03/03/1995");
  let date5 = String::from("24/12/2006");

  let mut date_queue = queue::ResizingArrayQueueOfStrings::new();

  date_queue.enqueue(date1);
  date_queue.enqueue(date2);
  date_queue.enqueue(date3);
  date_queue.enqueue(date4);
  date_queue.enqueue(date5);

  let mut date_vector: Vec<String> = Vec::new();

  while !date_queue.isEmpty() {
    date_vector.push(date_queue.dequeue());
  }

  println!("{:?}", date_vector);
}