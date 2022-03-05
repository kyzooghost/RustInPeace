#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// https://github.com/reneargento/algorithms-sedgewick-wayne/blob/master/src/chapter1/section3/Exercise19.java

pub mod queue {
  #[derive(Debug, Clone)]
  pub enum Node<T> {
    None,
    // Box are pointers. Live on stack, but store address to object on the heap
    // When you dereference them, they unbox their address' contents by following the pointer
    Node { item: T, next: Box<Node<T>>}
  }

  #[derive(Clone)]  
  pub struct Cursor<T> {
    curr: Node<T>
  }

  // Trait bound Copy to T
  // .. means we don't care about contents
  impl<T> Node<T> where T: Copy {
    pub fn new() -> Self {
      Self:: None
    }
    
    pub fn push(&mut self, x: T) {
      match self {
        Self::None => self.to_node(x),

        // Keep pushing x until it finds a tail?
        Self::Node {next, ..} => next.push(x)
      }
    }

    // Return Option, so can handle empty list
    // Mutable reference to self, means moving values out will be difficult
    pub fn pop(&mut self) -> Option<T> {
      match self {
        Self::None => None,

        Self::Node {item, next} => {
          let mut n = Box::new(Self::None); // create mutable pointer to empty Node 
          let item = *item; // Create new variable item, store item in here
          std::mem::swap(next, &mut n); // Swap places in memory, on next and reference to empty Box
          self.to_next(*n); // Dereference box to get Node value, pass to to_next
          Some(item) // Wrap item, return it
        }
      }
    }

    pub fn to_node(&mut self, x: T) {
      *self = match self {
        Self::None => {
          Self::Node {
            item: x,
            next: Box::new(Self::None )
          }
        },
        _ => panic!("Couldn't convert to Link!")
      }
    }

    pub fn to_none(&mut self) {
      *self = std::mem::replace(self, Node::None)
    }

    pub fn to_next(&mut self, nxt: Node<T>) {
      *self = nxt;
    }
  }
  
  impl<T> IntoIterator for Node<T> where T: Copy {
    type Item = T; // Tells compiler what to expect for iterator values
    type IntoIter = Cursor<T>; // Custom Cursor type

    // Must return whatever IntoIter refers to
    fn into_iter(self) -> Self::IntoIter {
      Cursor {
        curr: self
      }
    }
  }

  impl<T> Iterator for Cursor<T> where T: Copy {
    type Item = T;

    fn next(&mut self) -> Option<T> {
      let nxt = match self.curr {
        Node::None => None,

        Node::Node { item, ref mut next } => {
          let mut n = Box::new(Node::None);
          std::mem::swap(next, &mut n);
          self.curr = *n;
          Some(item)
        }
      };
      nxt
    }
  }
}

// p164
// 1.3.19 
// Give a code fragment that removes the last node in a linked list whose first node is first.

// Linked list implementation is FIFO

fn main() {
  let mut list: queue::Node<i32> = queue::Node::new();

  list.push(1);
  list.push(2);
  list.push(3);
  list.push(4);
  // First node now has value = 4, last node now has value = 1

  for i in list.clone() {
    println!("{:?}", list.pop().unwrap()); // node with value = 1, will be popped first
  }
}