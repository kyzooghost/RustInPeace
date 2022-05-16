#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// https://github.com/reneargento/algorithms-sedgewick-wayne/blob/master/src/chapter1/section3/Exercise19.java

pub mod LinkedList {
  #[derive(Debug, Clone)]
  pub enum Node<T> {
    None,
    Node { item: T, next: Box<Node<T>>}
  }

  // Pointer to a node
  #[derive(Clone)]  
  pub struct Cursor<T> {
    curr: Node<T>
  }

  impl<T> Node<T> where T: Copy + std::fmt::Debug {
    pub fn new() -> Self {
      Self::None
    }
    

    pub fn queue(&mut self, x: T) {
      match self {
        Self::None => self.to_node(x),
        Self::Node {next, ..} => next.queue(x)
      }
    }

    pub fn dequeue(&mut self) -> Option<T> {
      match self {
        Self::None => None,

        Self::Node {item, next} => {
          let mut n = Box::new(Self::None);
          let item = *item;
          std::mem::swap(next, &mut n);
          self.to_next(*n);
          Some(item)
        }
      }
    }

    // self is a mutable ref to Node<T>
    pub fn pop(&mut self) -> Option<T> {
      
      match self {
        Self::None => None,

        Self::Node {item, next} => {
          // next is a mutable reference to Node.next
          // Deref once to deref mutable reference
          // next is a Box, so deref again
          // Borrow the value of next
          let next = &mut **next;          
          let item = *item;

          // println!("{:?}", next);
          
          match next {
            // If get to last element
            Self::None => {
              *self = Self::None;
              Some(item)
            },
            
            Self::Node {..} => {
              // Should be recursive call on self.next
              // Promises to return Option<T> eventually lol
              next.pop()
            }
          }
        }
      }
    }

    pub fn to_node(&mut self, x: T) {
      *self = match self {
        Self::None => {
          Self::Node {
            item: x,
            next: Box::new(Self::None)
          }
        },
        _ => panic!("Couldn't convert to Link!")
      }
    }

    pub fn to_next(&mut self, nxt: Node<T>) {
      *self = nxt;
    }
  }
  
  impl<T> IntoIterator for Node<T> where T: Copy {
    type Item = T; 
    type IntoIter = Cursor<T>;

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

// Implementation from https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676 is like a Queue data structure
// FIFO head is 'first' and tail is 'last'
// Pop currently removes the head, so more like dequeue
// Need to remove from the other end

fn main() {
  let mut list: LinkedList::Node<i32> = LinkedList::Node::new();
  list.queue(1);
  list.queue(2);
  list.queue(3);
  list.queue(5);
  println!("{:?}", list.pop());
  println!("{:?}", list.pop());
  println!("{:?}", list);
  
  for (_index, element) in list.into_iter().enumerate() {
    println!("{:?}", element);
  }

}