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

  impl<T> Node<T> where T: Clone + std::fmt::Debug + Eq {
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

    pub fn find(&self, x: &T) -> bool {
      match self {
        Self::Node {item, next} => {
          // Base case - first item is the item you are looking for
          if item == x {return true}

          let mut next_node = &**next;

          while !Node::is_none(next_node) {

            // Increment to the next
            if let Self::Node { item, next } = next_node {
              if item == x {return true}
              next_node = &**next;
            };
          }   

        },

        _ => return false
      }
      
      false
    }

    // Almost the same implementation as find(), except instead of returning true/false
    // Returns index of the found element, if true
    // Returns None, if not found
    pub fn findIndex(&self, x: &T) -> Option<usize> {
      let mut index: usize = 0;
      
      match self {
        Self::Node {item, next} => {
          // Base case - first item is the item you are looking for
          if item == x {return Some(index)}

          let mut next_node = &**next;

          while !Node::is_none(next_node) {

            // Increment to the next
            if let Self::Node { item, next } = next_node {
              index = index + 1;
              if item == x {return Some(index)}
              next_node = &**next;
            };
          }   

        },

        _ => return None
      }
      
      None
    }

    pub fn removeAfter(&mut self, x: &T) {
      if !self.find(x) {panic!("could not find Node")}
      let index = self.findIndex(x).unwrap();
      self.delete(index + 1);
    }
  }

  impl Node<i32> {
    pub fn max(&self) -> Option<i32> {
      let mut max:i32 = 0;

      match self {
        Self::Node {item, next} => {
          if item > &max {max = *item;}

          let mut next_node = &**next;

          while !Node::is_none(next_node) {

            // Increment to the next
            if let Self::Node { item, next } = next_node {
              if item > &max {max = *item}
              next_node = &**next;
            };
          }   

        },

        _ => return None
      }

      Some(max)
    }

    pub fn max_recursive(&self, mut max: i32) -> Option<i32> {
      match self {
        Self::Node {item, next} => {
          if item > &max {max = *item}
          next.max_recursive(max)
        },

        _ => return Some(max)
      }
    }
  }

}

// p165
// 1.3.27

// Write a method max() that takes a reference to the first node in a linked list as argument 
// and returns the value of the maximum key in the list. 
// Assume that all keys are positive integers, and return 0 if the list is empty.

// 1.3.28
// Develop a recursive solution for this

fn main() {
  let mut list = LinkedList::Node::new();
  list.queue(1);
  list.queue(2);
  list.queue(43);
  list.queue(3);
  println!("{:?}", list.max());
  println!("{:?}", list.max_recursive(0));
}

#[cfg(test)]
mod tests {
  use super::LinkedList;

  #[test]
  fn find_test() {
    let mut list = LinkedList::Node::new();
    list.queue(1);
    list.queue(2);
    list.queue(43);
    list.queue(3);
    assert_eq!(list.max(), Some(43));
    assert_eq!(list.max_recursive(0), Some(43));
  }
}
