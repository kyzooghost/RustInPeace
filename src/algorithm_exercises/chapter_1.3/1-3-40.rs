#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ptr;

struct Node<T> {
    elem: T,
    next: *mut Node<T>,
    last: *mut Node<T>
}

pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
    size: usize
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut(), size: 0 }
    }

    pub fn insert_at_head(&mut self, elem: T) {
        if self.size == 0 {
            let new_head = Box::into_raw(
                                Box::new
                                    (Node {
                                        elem: elem, 
                                        next: ptr::null_mut(), 
                                        last: ptr::null_mut()
                                    })
            );

            self.head = new_head;
            self.tail = new_head;
        } else {
            let new_head = Box::into_raw(
                                Box::new
                                    (Node {
                                        elem: elem, 
                                        next: ptr::null_mut(), 
                                        last: self.head
                                    })
            );

            unsafe {
                (*self.head).next = new_head;
            }

            self.head = new_head;
        } 

        self.size = self.size + 1
    }

    pub fn insert_at_tail(&mut self, elem: T) {
        if self.size == 0 {
            let new_tail = Box::into_raw(
                                Box::new
                                    (Node {
                                        elem: elem, 
                                        next: ptr::null_mut(), 
                                        last: ptr::null_mut()
                                    })
            );

            self.head = new_tail;
            self.tail = new_tail;
        } else {
            let new_tail = Box::into_raw(
                                Box::new
                                    (Node {
                                        elem: elem, 
                                        next: self.tail, 
                                        last: ptr::null_mut()
                                    })
            );

            unsafe {
                (*self.tail).last = new_tail;
            }

            self.tail = new_tail;
        }

        self.size = self.size + 1
    }

    pub fn remove_from_head(&mut self) -> Option<T> {
        if self.size == 0 {return None}
        else {
            unsafe {
                let removed_head = Box::from_raw(self.head);

                if self.size == 1 {
                    self.head = ptr::null_mut();
                    self.tail = ptr::null_mut();
                } else {
                    self.head = removed_head.last;
                    (*self.head).next = ptr::null_mut();
                }

                self.size = self.size - 1;
                Some(removed_head.elem)
            }
        }
    }

    pub fn remove_from_tail(&mut self) -> Option<T> {
        if self.size == 0 {return None}
        else {
            unsafe {
                let removed_tail = Box::from_raw(self.tail);

                if self.size == 1 {
                    self.head = ptr::null_mut();
                    self.tail = ptr::null_mut();
                } else {
                    self.tail = removed_tail.next;
                    (*self.tail).last = ptr::null_mut();
                }

                self.size = self.size - 1;
                Some(removed_tail.elem)
            }
        }
    }

    // We count with index 0 => tail, 'index == self.size - 1' => head
    pub fn remove_at_index(&mut self, index: usize) -> Option<T> {
        if index + 1 > self.size {return None}
        else if index == 0 {self.remove_from_tail()}
        else if index + 1 == self.size {self.remove_from_head()}
        else {
            
            unsafe {
                let mut pointer_to_node_to_remove = self.tail;

                for _ in 0..index {
                    pointer_to_node_to_remove = (*pointer_to_node_to_remove).next;
                }

                let node_to_remove = Box::from_raw(pointer_to_node_to_remove);

                // Transfer node_to_remove.last to next node                
                (*node_to_remove.last).next = node_to_remove.next;
                
                // Transfer node_to_remove.next to last node.
                (*node_to_remove.next).last = node_to_remove.last;

                self.size = self.size - 1;
                Some(node_to_remove.elem)
            }
        }
    }

    // We count with index 0 => tail, 'index == self.size - 1' => head
    // This will insert elem into the linked list at index, and will right-shift any existing elements at index..head
    pub fn insert_at_index(&mut self, index: usize, elem: T) {
        if index > self.size {panic!("cannot insert at this index")}
        else if index == 0 {self.insert_at_tail(elem)}
        else if index == self.size {self.insert_at_head(elem)}
        else {
            unsafe {
                let mut pointer_to_current_node_at_index = self.tail;

                for _ in 0..index {
                    pointer_to_current_node_at_index = (*pointer_to_current_node_at_index).next;
                }
                
                let new_node = Box::into_raw(
                                    Box::new
                                        (Node {
                                            elem: elem, 
                                            next: pointer_to_current_node_at_index, 
                                            last: (*pointer_to_current_node_at_index).last
                                        })
                );

                (*(*pointer_to_current_node_at_index).last).next = new_node;
                (*pointer_to_current_node_at_index).last = new_node;

                self.size = self.size + 1;
            }

        }
    }

    pub fn peek_at_head(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut_at_head(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn peek_at_tail(&self) -> Option<&T> {
        unsafe {
            self.tail.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut_at_tail(&mut self) -> Option<&mut T> {
        unsafe {
            self.tail.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.tail.as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.tail.as_mut() }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.remove_from_tail() { }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_from_tail()
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

// 1.3.40 Read in a sequence of characters from standard input 
// and maintain the characters in a linked list with no duplicates. 
// When you read in a previously unseen character, insert it at the front of the list. 
// When you read in a duplicate character, delete it from the list and reinsert it at the beginning. 
// Name your program MoveToFront: it implements the well-known move-to-front strategy, 
// which is useful for caching, data compression, and many other applications 
// where items that have been recently accessed are more likely to be reaccessed.

struct MoveToFront {
    list: List<char>
}

impl MoveToFront {
    pub fn new() -> Self {
        MoveToFront { list: List::new() }
    }

    pub fn insert(&mut self, elem: char) {
        let search_result = self.list.iter().position(|x| x == &elem);

        match search_result {
            Some(index) => {
                self.list.remove_at_index(index);
                self.list.insert_at_tail(elem)
            },
            None => self.list.insert_at_tail(elem)
        }
    }

    pub fn into_iter(&self) -> Iter<char> {
        self.list.iter()
    }
}


fn main () {
    let mut list = MoveToFront::new();

    list.insert('a');
    list.insert('b');
    list.insert('c');
    list.insert('a');

    let mut iter = list.into_iter();

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
}
