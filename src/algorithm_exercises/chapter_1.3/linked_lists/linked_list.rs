#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676

pub mod queue {
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

fn main() {
  let mut list: queue::Node<i32> = queue::Node::new();
  let mut list2: queue::Node<i32> = queue::Node::new();

  list.push(1);
  list.push(2);
  list.push(3);
  list.push(4);

  list2.push(10);
  list2.push(20);
  list2.push(30);

  println!("{:?}", list2.pop().unwrap());
  println!("{:?}", list2.pop().unwrap());
  println!("{:?}", list2.pop().unwrap());
  println!{"---"};

  for i in list.clone() {
    println!("{}", i);
  }

  for i in list.clone().into_iter().map(|x| x * 2) {
    println!("{:?}", i);
  }

  // Can now iterate through
  for (i, x) in list.into_iter().enumerate() {
    println!("iter2: {} {}", i, x);
  }
}