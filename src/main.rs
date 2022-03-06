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

  impl<T> Node<T> where T: Copy {
    pub fn new() -> Self {
      Self::None
    }
    

    pub fn push(&mut self, x: T) {
      match self {
        Self::None => self.to_node(x),
        Self::Node {next, ..} => next.push(x)
      }
    }

    pub fn pop(&mut self) -> Option<T> {
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

// Linked list implementation is FIFO

fn main() {

}