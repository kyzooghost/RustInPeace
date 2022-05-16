#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod LinkedList {

  // Pointer to a node
  #[derive(Clone)]  
  pub struct Cursor<T> {
    curr: Node<T>
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

  trait Default {
    fn create() -> Self;
  }

  #[derive(Debug, Clone)]
  pub enum Node<T> {
    None,
    Node { item: T, next: Box<Node<T>>}
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
  }
}

// p164
// 1.3.20
// Write a method delete() that takes an int argument k and deletes the kth element in a linked list, if it exists.

fn main() {}

#[cfg(test)]
mod tests {
  use super::LinkedList;

  #[test]
  fn delete() {
    let mut list: LinkedList::Node<i32> = LinkedList::Node::new();
    list.queue(1);
    list.queue(2);
    list.queue(3);
    list.queue(5);
    assert_eq!(list.size(), 4);
    assert_eq!(list.delete(2), 3);
    assert_eq!(list.size(), 3);

    let mut list2: LinkedList::Node<i32> = LinkedList::Node::new();
    list2.queue(1);
    list2.queue(2);
    list2.queue(3);
    list2.queue(5);
    assert_eq!(list2.size(), 4);
    assert_eq!(list2.delete(1), 2);
    assert_eq!(list2.size(), 3);

    let mut list3: LinkedList::Node<i32> = LinkedList::Node::new();
    list3.queue(1);
    list3.queue(2);
    list3.queue(3);
    list3.queue(5);
    assert_eq!(list3.size(), 4);
    assert_eq!(list3.delete(3), 5);
    assert_eq!(list3.size(), 3);
    println!("{:?}", list3);
  }
}
