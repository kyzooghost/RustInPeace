#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod LinkedList {

  // Pointer to a node
  #[derive(Clone)]  
  pub struct Cursor<T> {
    curr: Node<T>
  }

  impl<T> IntoIterator for Node<T> where T: Clone {
    type Item = T; 
    type IntoIter = Cursor<T>;

    fn into_iter(self) -> Self::IntoIter {
      Cursor {
        curr: self
      }
    }
  }

  impl<T> Iterator for Cursor<T> where T: Clone {
    type Item = T;

    fn next(&mut self) -> Option<T> {
      let nxt = match std::mem::replace(&mut self.curr, Node::None) {
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

  trait Default {
    fn create() -> Self;
  }

  #[derive(Debug, Clone)]
  pub enum Node<T> {
    None,
    Node { item: T, next: Box<Node<T>>}
  }

  impl<T> Node<T> where T: Clone + std::fmt::Debug {
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
      match std::mem::replace(self, Self::None) {
        Self::None => None,

        Self::Node {item, ref mut next} => {
          let mut n = Box::new(Self::None);
          let item = item;
          std::mem::swap(next, &mut n);
          self.to_next(*n);
          Some(item)
        }
      }
    }

    pub fn pop(&mut self) -> Option<T> {
      // Destructure self
      if let Self::Node {item, ref mut next} = self {
        match **next {
          Self::Node {..} => {
            next.pop()
          }, 

          _ => {
            let item = item.clone();
            *self = Self::None;
            Some(item)
          }
        }
      } else {unimplemented!()}
    }

    pub fn delete(&mut self, k: usize) -> T {
      // We start from index 0
      assert!(k <= self.size() - 1, "k cannot be bigger than linked list size");
      assert!(self.size() > 0, "cannot delete from None");

      match k {
        0 => {
          self.dequeue().unwrap()
        },

        _ => {
          // Seems like I need an 'if let...else' block to destructure an Enum variant
          // So defining a struct inside of an Enum like this is clumsy
          if let Self::Node {next, ..} = self {
            
            let mut next_to_change = next;

            // Make next_to_change point to the right next
            for _i in 0..k-1 {
              match **next_to_change {
                Self::Node {ref mut next, ..} => {
                  next_to_change = next;
                },
                _ => {}
              }
            }

            // Mutate *next_to_change to the subsequent next
            let next_to_change_clone = next_to_change.clone();

            match *next_to_change_clone {
              Self::Node {item, next} => {
                *next_to_change = next;
                return item
              },
              _ => {}
            }
            
            unimplemented!()
            } else {unimplemented!()}
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
        _ => panic!("Couldn't convert to Node!")
      }
    }

    pub fn to_next(&mut self, nxt: Node<T>) {
      *self = nxt;
    }

    pub fn is_none(node: &Self) -> bool {
      match node {
        Self::None => true,
        _ => false
      }
    }

    pub fn size(&self) -> usize {
      let mut size: usize = 0;

      match self {
        Self::None => 0,

        Self::Node {next, ..} => {
          size = size + 1;

          // If single isolated node
          if Node::is_none(&**next) {return size}

          let mut next_node = &**next;

          // While next does not refer to none box
          while !Node::is_none(next_node) {
            size = size + 1;

            // Increment to the next
            if let Self::Node { next, .. } = next_node {
              next_node = &**next;
            };
          }   

          size
        }
      }
    }
  }

}

// p164
// 1.3.21
// Write a method find() that takes a linked list and a string key as arguments 
// and returns true if some node in the list has key as its item field, false otherwise.

// First enable LinkedList of string

fn main() {}

fn find (linkedlist: LinkedList::Node<&str>, key: &str) -> bool {
  for node in linkedlist {
    if node == key {return true}
  }
  
  false
}

#[cfg(test)]
mod tests {
  use super::LinkedList;
  use super::find;

  #[test]
  fn find_test() {
    let mut list: LinkedList::Node<&str> = LinkedList::Node::new();
    list.queue("a");
    list.queue("b");
    list.queue("c");
    list.queue("d");
    list.queue("e");
    assert_eq!(find(list.clone(), "a"), true);
    assert_eq!(find(list, "z"), false);
  }
}
