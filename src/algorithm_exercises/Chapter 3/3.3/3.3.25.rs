#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p434 rotateLeft(), rotateRight()
// p436 flipColors()
// p439 put()
// p451 Exercises

// 3.2.25

// Top-down 2-3-4 trees. 
// Develop an implementation of the basic symbol-table API that uses balanced 2-3-4 trees as the underlying data structure, 
// using the red-black representation and the insertion method described in the text, 
// where 4-nodes are split by flipping colors on the way down the search path and balancing on the way up.

use std::ptr;

#[derive(PartialEq, Clone)]
enum Colour {
    Red,
    Black
}

pub struct Node<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> {
    key: T,
    value: U,
    subtree_size: usize,
    left_child: *mut Node<T, U>,
    right_child: *mut Node<T, U>,
    colour: Colour
}

pub struct BinarySearchTree<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> {
    pub root: *mut Node<T, U>,
}

pub struct IntoIter<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy>(BinarySearchTree<T, U>);

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> BinarySearchTree<T, U> {
    pub fn size(&self) -> usize {self._subtree_size(self.root)}

    pub fn is_empty(&self) -> bool {self.size() == 0}

    fn _subtree_size(&self, node_pointer: *mut Node<T, U>) -> usize {
        if node_pointer.is_null() {return 0}

        unsafe { 
            (*node_pointer).subtree_size
        }
    }

    pub fn new() -> Self {
        BinarySearchTree { root: ptr::null_mut() }
    }

    fn isRed(&self, node_pointer: *mut Node<T, U>) -> bool {
        if node_pointer.is_null() {return false}
        (*node_pointer).colour == Colour::Red
    }

    fn rotateLeft(&mut self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        assert!(!(*node_pointer).right_child.is_null() && self.isRed((*node_pointer).right_child), "cannot rotate left if have no red right child");

        let new_parent = (*node_pointer).right_child;
        (*node_pointer).right_child = (*new_parent).left_child;
        (*new_parent).left_child = node_pointer;

        (*new_parent).colour = (*node_pointer).colour;
        (*node_pointer).colour = Colour::Red;

        (*new_parent).subtree_size = (*node_pointer).subtree_size;
        (*node_pointer).subtree_size = 1 + self._subtree_size((*node_pointer).left_child) + self._subtree_size((*node_pointer).right_child);
        
        return new_parent
    }

    fn rotateRight(&mut self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        assert!(!(*node_pointer).left_child.is_null() && self.isRed((*node_pointer).left_child), "cannot rotate right if have no red left child");

        let new_parent = (*node_pointer).left_child;
        (*node_pointer).left_child = (*new_parent).right_child;
        (*new_parent).right_child = node_pointer;

        (*new_parent).colour = (*node_pointer).colour;
        (*node_pointer).colour = Colour::Red;

        (*new_parent).subtree_size = (*node_pointer).subtree_size;
        (*node_pointer).subtree_size = 1 + self._subtree_size((*node_pointer).left_child) + self._subtree_size((*node_pointer).right_child);

        return new_parent
    }

    fn flipColours(&mut self, node_pointer: *mut Node<T, U>) {

        if (*node_pointer).left_child.is_null() || (*node_pointer).right_child.is_null() || node_pointer.is_null() {
            return
        }

        if 
            self.isRed((*node_pointer).left_child) &&
            self.isRed((*node_pointer).right_child) &&
            !self.isRed(node_pointer)
        {
                (*node_pointer).colour = Colour::Red;
                (*(*node_pointer).left_child).colour = Colour::Black;
                (*(*node_pointer).right_child).colour = Colour::Black;
        } 
    }

    pub fn get(&self, key: T) -> Option<U> {
        if self.is_empty() {return None}

        // Recursive search for key, start from root
        self._get(self.root, key)
    }

    // Return value associated with key in the subtree rooted at node_pointer
    // If key not present, return None
    fn _get(&self, node_pointer: *mut Node<T, U>, key: T) -> Option<U> {
        if self.is_empty() {return None}

        // Recursive base case
        // Also guard against dereferencing null pointer
        if node_pointer.is_null() {return None}

        unsafe {
            // If given key < root key, recurse into left child
            if key < (*node_pointer).key {return self._get((*node_pointer).left_child, key)}
            // If given key > root key, recurse into right child
            else if key > (*node_pointer).key {return self._get((*node_pointer).right_child, key)}
            // Else given key == root key, search hit
            else {return Some((*node_pointer).value)}
        }
    }

    pub fn put(&mut self, key: T, value: U) {
        self.root = self._put(self.root, key, value); // ? Don't understand re-assigning self.root
        (*self.root).colour = Colour::Black;
    }

    // If search hit, change corresponding node value
    // Else, add new node
    // Recursive call here, keep re-assigning nodes on each level
    fn _put(&mut self, node_pointer: *mut Node<T, U>, key: T, value: U) -> *mut Node<T, U> {
        if node_pointer.is_null() {
            // If recurse into non-existent node, create a node and return it
            let new_node = Box::into_raw(
                Box::new
                    (Node {
                        key: key, 
                        value: value, 
                        subtree_size: 1,
                        left_child: ptr::null_mut(), 
                        right_child: ptr::null_mut(),
                        colour: Colour::Red
                    })
            );

            return new_node
        }

        // Flip colours on the way down
        if self.isRed((*node_pointer).left_child) && self.isRed((*node_pointer).right_child) {
            self.flipColours(node_pointer);
        }

        unsafe {
            // If given key < root key, recurse into left
            if key < (*node_pointer).key {
                (*node_pointer).left_child = self._put((*node_pointer).left_child, key, value)
            // Else if given key > root key, recurse into right
            } else if key > (*node_pointer).key {
                (*node_pointer).right_child = self._put((*node_pointer).right_child, key, value)
            // Else search hit
            } else {
                (*node_pointer).value = value;
            }

            // Balance on the way up
            if self.isRed((*node_pointer).right_child) && !self.isRed((*node_pointer).left_child) {
                node_pointer = self.rotateLeft(node_pointer);
            }

            if self.isRed((*node_pointer).left_child) && self.isRed((*(*node_pointer).left_child).left_child) {
                node_pointer = self.rotateRight(node_pointer);
            }

            // No colour flip here to break 4-nodes
            // So any 4-nodes it finds on the way down will be broken
            // However if the insertion creates a new 4-node, it is not broken

            // Increment counts
            (*node_pointer).subtree_size = self._subtree_size((*node_pointer).left_child) + self._subtree_size((*node_pointer).right_child) + 1;            
        }

        // Mmmm, intuitively this feels redundant in most cases
        return node_pointer
    }

    pub fn min(&self) -> Option<T> {
        if self.is_empty() {return None}
        unsafe {
            Some( (*self._min(self.root)).key )
        }
    }

    fn _min(&self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        // Keep recursing down left subchild, until hit null pointer
        unsafe {
            if (*node_pointer).left_child.is_null() {return node_pointer}
            return self._min((*node_pointer).left_child)
        }
    }

    pub fn max(&self) -> Option<T> {
        if self.is_empty() {return None}
        unsafe {
            Some( (*self._max(self.root)).key )
        }
    }

    fn _max(&self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        // Keep recursing down right subchild, until hit null pointer
        unsafe {
            if (*node_pointer).right_child.is_null() {return node_pointer}
            return self._max((*node_pointer).right_child)
        }
    }

    pub fn floor(&self, key: T) -> Option<T> {
        let pointer = self._floor(self.root, key);
        if pointer.is_null() {return None}
        unsafe {
            return Some((*pointer).key)
        }
    }

    // Floor is biggest element <= key
    fn _floor(&self, node_pointer: *mut Node<T, U>, key: T) -> *mut Node<T, U> {
        if node_pointer.is_null() {return ptr::null_mut()}

        unsafe {
            // Search hit
            if key == (*node_pointer).key {return node_pointer}

            // If key < pointer key, recurse into left child
            else if key < (*node_pointer).key {return self._floor((*node_pointer).left_child, key)}
            
            // Else if key > pointer key, floor could be in right_child
            let floor_pointer = self._floor((*node_pointer).right_child, key);
            if !floor_pointer.is_null() {return floor_pointer} // If found floor, return it
            else {return node_pointer} // Else return current pointer
        }
    }

    pub fn ceiling(&self, key: T) -> Option<T> {
        let pointer = self._ceiling(self.root, key);
        if pointer.is_null() {return None}
        unsafe {
            return Some((*pointer).key)
        }
    }

    fn _ceiling(&self, node_pointer: *mut Node<T, U>, key: T) -> *mut Node<T, U> {
        if node_pointer.is_null() {return ptr::null_mut()}

        unsafe {
            if key == (*node_pointer).key {return node_pointer}
            else if key > (*node_pointer).key {return self._ceiling((*node_pointer).right_child, key)}
            
            let ceiling_pointer = self._ceiling((*node_pointer).left_child, key);
            if !ceiling_pointer.is_null() {return ceiling_pointer}
            else {return node_pointer}
        }
    }

    pub fn select(&self, rank: usize) -> Option<T> {
        if self.is_empty() || rank >= self.size() {return None}
        unsafe {
            Some( (*self._select(self.root, rank)).key )
        }
    }

    fn _select(&self, node_pointer: *mut Node<T, U>, rank: usize) -> *mut Node<T, U> {
        if node_pointer.is_null() {return ptr::null_mut()}

        unsafe {
            let left_subtree_size = self._subtree_size((*node_pointer).left_child);
            if left_subtree_size > rank {return self._select((*node_pointer).left_child, rank)}
            else if left_subtree_size < rank {return self._select((*node_pointer).right_child, rank - left_subtree_size - 1)}
            else {return node_pointer}
        }
    }

    pub fn rank(&self, key: T) -> usize {
        self._rank(key, self.root)
    }

    fn _rank(&self, key: T, node_pointer: *mut Node<T, U>) -> usize {
        if node_pointer.is_null() {return 0}

        unsafe {
            // Return rank in left subchild
            if key < (*node_pointer).key {return self._rank(key, (*node_pointer).left_child)}
            // Else left subchild + 1 (for pointer) + rank in right branch
            else if key > (*node_pointer).key {return 1 + self._subtree_size((*node_pointer).left_child) + self._rank(key, (*node_pointer).right_child)}
            // If search hit, return # of nodes in left branch
            else {return self._subtree_size((*node_pointer).left_child)}
        }
    }

    pub fn peek_at_root(&self) -> Option<(&T, &U)> {
        if self.size() == 0 {return None}
        unsafe {
            self.root.as_ref().map(|node| (&node.key, &node.value))
        }
    }

    pub fn peek_mut_at_root(&mut self) -> Option<(&mut T, &mut U)> {
        if self.size() == 0 {return None}
        unsafe {
            self.root.as_mut().map(|node| (&mut node.key, &mut node.value))
        }
    }

    pub fn into_iter(self) -> IntoIter<T, U> {
        IntoIter(self)
    }

    pub fn height(&self) -> usize {
        // Start from root
        // Can either go down left subchild or right subchild, go down subchild with most children
        // With each recursion, add height by 1
        self._height(self.root, 0)
    }

    pub fn _height(&self, pointer: *mut Node<T, U>, accum_height: usize) -> usize {
        if pointer.is_null() {return accum_height}

        unsafe {
            if (*pointer).left_child.is_null() {return self._height((*pointer).right_child, accum_height + 1)}
            else if (*pointer).right_child.is_null() {return self._height((*pointer).left_child, accum_height + 1)}
            // Both left and right child are present, choose child with bigger # of subchildren
            else {
                if (*(*pointer).left_child).subtree_size > (*(*pointer).right_child).subtree_size {
                    return self._height((*pointer).left_child, accum_height + 1)
                } else {
                    return self._height((*pointer).right_child, accum_height + 1)
                }
            }
        }
    }

    pub fn all_keys(&self) -> Vec<T> {
        assert!(!self.is_empty(), "empty tree has no range");
        self.keys(self.min().unwrap(), self.max().unwrap())
    }

    pub fn keys(&self, low_key: T, high_key: T) -> Vec<T> {
        let mut key_vec: Vec<T> = Vec::new();
        self._keys(self.root, &mut key_vec, low_key, high_key);
        key_vec
    }

    fn _keys(&self, pointer: *mut Node<T, U>, key_vec: &mut Vec<T>, low_key: T, high_key: T) {
        if pointer.is_null() {return}

        unsafe {
            if low_key < (*pointer).key {
                self._keys((*pointer).left_child, key_vec, low_key, high_key)
            } 
            
            if low_key <= (*pointer).key && high_key >= (*pointer).key {
                key_vec.push((*pointer).key);
            } 
            
            if high_key > (*pointer).key {
                self._keys((*pointer).right_child, key_vec, low_key, high_key)
            }
        }
    }

}

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> Drop for BinarySearchTree<T, U> {
    fn drop(&mut self) {
        while !self.is_empty() {self.deleteMin()}
    }
}

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> Iterator for IntoIter<T, U> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let min = self.0.min();
        self.0.deleteMin();
        min
    }
}

fn main() {
}
