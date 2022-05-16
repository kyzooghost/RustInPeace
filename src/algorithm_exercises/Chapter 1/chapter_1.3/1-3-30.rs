#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly-2022-01-21 miri run

use std::ptr;

pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: usize
}

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut(), size: 0 }
    }
    pub fn push(&mut self, elem: T) {
      self.size = self.size + 1;  
      unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                elem: elem,
                next: ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                self.size = self.size - 1;
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)
            }
        }
    }

    pub fn size(&self) -> usize {
      self.size
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.head.as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.head.as_mut() }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

// 1.3.30 - Write a function that takes the first Node in a linked list as argument and (destructively) reverses the list
//, returning the first Node in the result.

// I can't be bothered rolling my own linked list for this

// Just push onto a stack, and pop-off

fn main () {
  let mut list = List::new();
  list.push(1);
  list.push(2);
  list.push(3);
  list.push(4);
  // println!("{:?}", list.pop());
  list = reverse_list(list);
  // println!("{:?}", list.pop());
  // println!("{:?}", list.pop());

}

fn reverse_list<T>(mut list: List<T>) -> List<T> {
  if list.size == 0 {panic!("cannot reverse a 0 list")}
  else if list.size == 1 {return list}
  else {
    let mut vec = Vec::new();
  
    while list.size > 0 {
      vec.push(list.pop());
    }

    while vec.len() > 0 {
      list.push(vec.pop().unwrap().unwrap());
    }

    return list
  }
}