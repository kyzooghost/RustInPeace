#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// https://medium.com/swlh/implementing-a-linked-list-in-rust-c25e460c3676

// Push value into list -> Create new node that linked to tail
// -> Previous tail becomes becomes node with new value as its next
// 1 -> 2 -> 3
// Push 4, 1 -> 2 -> 3 -> 4
// Looks like a queue data structure

pub mod queue {
  // A Node can either exist or not exist, use Enum to create your own 'undefined' type
  #[derive(Debug, Clone)]
  pub enum Node<T> {
    None,
    // Box are pointers. Live on stack, but store address to object on the heap
    // When you dereference them, they unbox their address' contents by following the pointer
    // Node contain 'item' and pointer (Box) to another Node
    Node { item: T, next: Box<Node<T>>}
  }

  // Pointer to a node
  #[derive(Clone)]  
  pub struct Cursor<T> {
    curr: Node<T>
  }

  // T is trait-bound to Copy
  // .. means we don't care about contents
  // Can implement Copy for any value that live on the stack
  impl<T> Node<T> where T: Copy {
    pub fn new() -> Self {
      Self::None
    }

    // PUSH 1 | 1, -> null
    // PUSH 2 | 1 -> 2 -> null
    // PUSH 3 | 1 -> 2 -> 3 -> null
    
    pub fn push(&mut self, x: T) {
      match self {
        // If None, to_node(x)
        Self::None => self.to_node(x),

        // Recursive until it hits Self::None branch, 
        Self::Node {next, ..} => next.push(x)
      }
    }

    // Return Option, so can handle empty list
    // Option => None or Some(T)
    // Mutable reference to self, means moving values out will be difficult
    pub fn pop(&mut self) -> Option<T> {
      match self {
        Self::None => None,

        Self::Node {item, next} => {
          let mut n = Box::new(Self::None); // create mutable pointer to empty Node 
          let item = *item; // Move item into a new variable, out of struct self 
          std::mem::swap(next, &mut n); 
          // Swap places in memory, on next and reference to empty Box
          // n becomes a mutable reference to next, next becomes a pointer to null
          self.to_next(*n); // Dereference box to get Node value, pass to to_next
          // self becomes next (pop from the front, so FIFO queue)
          Some(item) // Wrap item, return it

          // overall 'item' moved out into new variable, wrapped and returned
          // next is swapped with Box::new(Self::None)
          // self itself becomes next dereferenced
        }
      }
    }

    pub fn to_node(&mut self, x: T) {
      // Need to dereference the reference, to change self
      // But to access self, we don't need to dereference
      // match itself will return a value, cases for 'self'
      // self == Self::None => self becomes new Node
      // else, panic!
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
      // Node::None moved to where 'self' was in memory
      // self moved to where Node::None was in memory
      // So *self - self is now random detached spot in memory, so *self is too
      // *self = std::mem::replace(self, Node::None)
      *self = std::mem::replace(self, Node::None)
    }

    pub fn to_next(&mut self, nxt: Node<T>) {
      *self = nxt;
    }
  }

  // By implementing IntoIterator for a type, define how it will be converted into an iterator
  impl<T> IntoIterator for Node<T> where T: Copy {
    type Item = T; // Tells compiler what to expect for iterator values
    type IntoIter = Cursor<T>; // Custom Cursor type, or Iterator type

    // Must return whatever IntoIter refers to
    fn into_iter(self) -> Self::IntoIter {
      Cursor {
        curr: self
      }
    }
  }

  impl<T> Iterator for Cursor<T> where T: Copy {
    type Item = T;

    // Advance iterator
    // Return None when iteration finished
    fn next(&mut self) -> Option<T> {
      // What node does the cursor currently point at?
      let nxt = match self.curr {
        Node::None => None,

        // Node instances here are not mutably borrowed, but values
        // We are destructuring Node struct here
        // ref mut => take a mutable reference to 'next' field
        // Indicate that we want to mutate a field of the struct
        Node::Node { item, ref mut next } => {
          // Create mutable pointer to Node::None
          let mut n = Box::new(Node::None);
          // Put Box::None into next, next into "n"
          std::mem::swap(next, &mut n);
          // self.curr now mutated to Node pointed at by next (destructured the pointer) 
          // Cursor moves to next element
          self.curr = *n;

          // Return wrapped item
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

  // Implementing Iterator enable easy cloning
  // Don't have to consume the list to use its values
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