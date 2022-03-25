#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Need a circular linked list
// Need to be able to point 'next' to the node itself
// Don't need to enable this in the LinkedList method itself, but need to be possible through Queue

// MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly-2022-01-21 miri run

pub mod LinkedList {
  use std::marker::PhantomPinned;
  use std::ptr;
  use std::mem::ManuallyDrop;

  // _pin -> make the struct implement !Unpin, and make it pinnable
  #[derive(Debug)]
  pub struct Node<T> where T: std::fmt::Debug {
      value: T,
      pub next: *mut Node<T>,
      _pin: PhantomPinned,
  }

  #[derive(Debug)]
  pub struct CircularLinkedList<T> where T: std::fmt::Debug {
    pub last: *mut Node<T>,
    size: usize
  }

  pub struct IntoIter<T>(CircularLinkedList<T>) where T: std::fmt::Debug;

  impl<T> CircularLinkedList<T> where T: std::fmt::Debug {
      pub fn new() -> Self {
        CircularLinkedList { last: ptr::null_mut(), size: 0 }
      }

      pub fn enqueue(&mut self, value: T) {
        self.size = self.size + 1;

        // Create a new Node struct, assign it to new_last variable
        // Struct - no guarantees on data layout
        let new_last = Node {
          value,
          next: ptr::null_mut(),
          _pin: PhantomPinned,
        };
        
        // Construct a new <Pin<Box<T>>>
        // new_last is pinned in heap, and can't move from this address
        // pinned_new_last is Pin<Box<T>> type - a smart pointer
        // Wrap in ManuallyDrop - 0-cost wrapper that stop compiler from calling destructor for pinned_new_last
        // When Pin<Box<T>> dropped, both the pointer and pointee dropped, so need to clean this up in the drop
        let mut pinned_new_last = ManuallyDrop::new(Box::pin(new_last));

        if self.last.is_null() {          
          unsafe {
            // as_mut gets Pin<Box<Node>> => Pin<&mut Node>
            // get_unchecked_mut gets Pin<&mut Node> => &mut Node
            // We coerce it into a *mut T
            pinned_new_last.as_mut().get_unchecked_mut().next = pinned_new_last.as_mut().get_unchecked_mut() as *mut Node<T>; 
            self.last = pinned_new_last.as_mut().get_unchecked_mut() as *mut Node<T>;
          }
        } else {
          unsafe {
            // pinned_new_last.as_mut().get_unchecked_mut().next = (*self.last).next; 
            pinned_new_last.as_mut().get_unchecked_mut().next = self.last; 

            // (*self.last).next = pinned_new_last.as_mut().get_unchecked_mut() as *mut Node<T>;
   
            // while loop to the first node
            // Change the pointer on the first node to pinned_new_last
            // first.next = self.last
            let mut first = self.last;

            while (*first).next != self.last {
              first = (*first).next;
            }

            // Deref first, alter next field to pinned_new_last
            (*first).next = pinned_new_last.as_mut().get_unchecked_mut() as *mut Node<T>;

            ////

            self.last = pinned_new_last.as_mut().get_unchecked_mut() as *mut Node<T>;
          }
        }        

      }

      pub fn dequeue(&mut self) -> Option<T> {
        if self.last.is_null() {None} 
        else {
          self.size = self.size - 1;

          unsafe {

            // This fixed the memory leak!!!
            // Let Box destructor call destructor of T and free allocated memory
            let last = Box::from_raw(self.last);

            if self.size == 0 {self.last = ptr::null_mut();}
            else {self.last = last.next;}

            let value = (*last).value;
            Some(value)
          }
        }
      }        

      pub fn peek(&self) -> Option<&T> {
        
        if self.last.is_null() {None}

        else {
          unsafe {
            // Deref self.last (it is a *mut Node)
            // Get value field
            // Cast entire thing to &
            Some(&(*self.last).value)
          }
        }
        
      }

      pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
      }

  }

  impl<T> Drop for CircularLinkedList<T> where T: std::fmt::Debug  {
    fn drop(&mut self) {
      while let Some(_) = self.dequeue() {}
    }
  }

  impl<T> Iterator for IntoIter<T> where T: std::fmt::Debug {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
      // unsafe{
      //   println!("{:?}", (*self.0.last));
      // }
      self.0.dequeue()
    }
  }

}

fn main () {
  let mut list = LinkedList::CircularLinkedList::new();
  list.enqueue(1);
  list.enqueue(2);
  list.enqueue(3);
  list.enqueue(4);

  let mut iter = list.into_iter();
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
}
