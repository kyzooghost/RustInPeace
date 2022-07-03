#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p434 rotateLeft(), rotateRight()
// p436 flipColors()
// p439 put()
// p451 Exercises

// 3.3.41

// deleteMin(), deleteMax() and delete() for red-black BST

use std::ptr;

#[derive(PartialEq, Clone, Copy, Debug)]
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

pub struct RedBlackBST<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> {
    pub root: *mut Node<T, U>,
}

pub struct IntoIter<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy>(RedBlackBST<T, U>);

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> RedBlackBST<T, U> {
    pub fn size(&self) -> usize {self._subtree_size(self.root)}

    pub fn is_empty(&self) -> bool {self.size() == 0}

    fn _subtree_size(&self, node_pointer: *mut Node<T, U>) -> usize {
        if node_pointer.is_null() {return 0}

        unsafe { 
            (*node_pointer).subtree_size
        }
    }

    pub fn new() -> Self {
        RedBlackBST { root: ptr::null_mut() }
    }

    fn isRed(&self, node_pointer: *mut Node<T, U>) -> bool {
        unsafe {
            if node_pointer.is_null() {return false}
            (*node_pointer).colour == Colour::Red
        }
    }

    fn rotateLeft(&mut self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        unsafe {
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
        
    }

    fn rotateRight(&mut self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        unsafe {
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
    }

    fn flipColours(&mut self, node_pointer: *mut Node<T, U>) {
        unsafe {
            if (*node_pointer).left_child.is_null() || (*node_pointer).right_child.is_null() || node_pointer.is_null() {
                return
            }

            if self.isRed((*node_pointer).left_child) {
                (*(*node_pointer).left_child).colour = Colour::Black;
            } else {
                (*(*node_pointer).left_child).colour = Colour::Red;
            }

            if self.isRed((*node_pointer).right_child) {
                (*(*node_pointer).right_child).colour = Colour::Black;
            } else {
                (*(*node_pointer).right_child).colour = Colour::Red;
            }

            if self.isRed(node_pointer) {
                (*node_pointer).colour = Colour::Black;
            } else {
                (*node_pointer).colour = Colour::Red;
            }
            
    
            // if 
            //     self.isRed((*node_pointer).left_child) &&
            //     self.isRed((*node_pointer).right_child) &&
            //     !self.isRed(node_pointer)
            // {
            //         (*node_pointer).colour = Colour::Red;
            //         (*(*node_pointer).left_child).colour = Colour::Black;
            //         (*(*node_pointer).right_child).colour = Colour::Black;
            // } 
            // else if 
            //    !self.isRed((*node_pointer).left_child) &&
            //     !self.isRed((*node_pointer).right_child) &&
            //     self.isRed(node_pointer)
            // {
            //     (*node_pointer).colour = Colour::Black;
            //     (*(*node_pointer).left_child).colour = Colour::Red;
            //     (*(*node_pointer).right_child).colour = Colour::Red;
            // }
        }
    }

    fn moveRedLeft(&mut self, mut pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        // If no red on left, flip the colours
        self.flipColours(pointer);

        unsafe {
            // If we create a right => left consecutive red link situation, fix it
            // We don't care about left => left, or right => right because fixUp will fix?
            if !pointer.is_null() && 
                !(*pointer).right_child.is_null() &&
                self.isRed((*(*pointer).right_child).left_child) {
                // Convert to right => right red link
                (*pointer).right_child = self.rotateRight((*pointer).right_child);
                // Rotate left
                pointer = self.rotateLeft(pointer);
                // Flip colour - will keep one remaining red link on 2nd left
                self.flipColours(pointer);
            }
        }

        return pointer
    }

    fn moveRedRight(&mut self, mut pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        self.flipColours(pointer);
        unsafe {
            if self.isRed((*(*pointer).left_child).left_child) {
                pointer = self.rotateRight(pointer);
                self.flipColours(pointer);
            }
        }
        return pointer
    }

    // Check that no node is connected to two red links => Check both children are not red
    // Check no right-learning red links
    pub fn is23(&self) -> bool {
        self._is23(self.root)
    }

    pub fn _is23(&self, pointer: *mut Node<T, U>) -> bool {
        if pointer.is_null() {return true}

        unsafe {
            let bothChildrenNotRed = !self.isRed((*pointer).left_child) && !self.isRed((*pointer).right_child);
            let noRightLeaningRedLink = !self.isRed((*pointer).right_child);
                   
            bothChildrenNotRed && noRightLeaningRedLink && self._is23((*pointer).left_child) && self._is23((*pointer).right_child) 
        }

    }

    // Check that all paths from root to null link have same number of black links
    fn isBalanced(&self) -> bool {
        // Count black nodes from root to min
        let mut blackNodes = 0;
        let mut pointer = self.root;

        unsafe {
            while !pointer.is_null() {
                if !self.isRed(pointer) {blackNodes += 1;}
                pointer = (*pointer).left_child; 
            }
        }

        self._isBalanced(self.root, blackNodes)
    }

    fn _isBalanced(&self, pointer: *mut Node<T, U>, mut blackNodes: isize) -> bool {
        if pointer.is_null() {return blackNodes == 0}

        unsafe {
            if !self.isRed(pointer) {blackNodes -= 1}
            self._isBalanced((*pointer).left_child, blackNodes) && self._isBalanced((*pointer).right_child, blackNodes)
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
        self.root = self._put(self.root, key, value);
        unsafe {
            // Maintain root black
            (*self.root).colour = Colour::Black;
        }
    }

    // If search hit, change corresponding node value
    // Else, add new node
    // Recursive call here, keep re-assigning nodes on each level
    fn _put(&mut self, mut node_pointer: *mut Node<T, U>, key: T, value: U) -> *mut Node<T, U> {
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


        // Split four-node if create one at the bottom
        // If parent has a red other child, then flip parent and other child, and return Black Node            

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
            node_pointer = self.fixUp(node_pointer);

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
        if self.is_empty()  {return vec![]};
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

    fn fixUp(&mut self, mut pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        unsafe {
            // No right-leaning red links
            if self.isRed((*pointer).right_child) && !self.isRed((*pointer).left_child) {
                pointer = self.rotateLeft(pointer);
            }

            // No two consecutive-left leaning links => Convert to 1x left red + 1x right red
            if self.isRed((*pointer).left_child) && self.isRed((*(*pointer).left_child).left_child) {
                pointer = self.rotateRight(pointer);
            }

            // If 1x left red, 1x right red, flip colours
            if self.isRed((*pointer).left_child) && self.isRed((*pointer).right_child) {
                self.flipColours(pointer);
            }

            pointer
        }
    }

    fn deleteMin(&mut self) {
        if self.root.is_null() {return}
        unsafe {
            self.root = self._deleteMin(self.root);
            if self.root.is_null() {return}
            (*self.root).colour = Colour::Black;
        }
    }

    // Want to ensure we delete from 3-node
    // 3-node means have to delete from red node
    // If we delete from 3-node, balanced because no change in black height

    fn _deleteMin(&mut self, mut pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        unsafe {
            // Base case, reached null left link
            if (*pointer).left_child.is_null() {
                let _removed_node = Box::from_raw(pointer); // Free deleted node, prevent memory leak
                return ptr::null_mut()
            }
            
            // If no left links are red, move red left
            if ( !self.isRed((*pointer).left_child) && (*pointer).left_child.is_null() ) ||
            ( !self.isRed((*pointer).left_child) && !self.isRed((*(*pointer).left_child).left_child) ) {
                pointer = self.moveRedLeft(pointer);
            }

            // Recurse into left child
            (*pointer).left_child = self._deleteMin((*pointer).left_child);

            // Fix-up on way-up
            self.fixUp(pointer)
        }
    }

    fn deleteMax(&mut self) {
        if self.root.is_null() {return}
        unsafe {
            self.root = self._deleteMax(self.root);
            if self.root.is_null() {return}
            (*self.root).colour = Colour::Black;
        }
    }

    fn _deleteMax(&mut self, mut pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        unsafe {

            // Need to rotate right any left-leaning red links, because otherwise can lose
            // a node if we reach situation where max node has a left child, and no right child
            // We then lose left child, if we do not enforce the invariant that no
            // left-leaning red node
            if self.isRed((*pointer).left_child) {
                pointer = self.rotateRight(pointer);
            }

            // Base case, reached null right link
            if (*pointer).right_child.is_null() {
                let _removed_node = Box::from_raw(pointer); // Free deleted node, prevent memory leak
                return ptr::null_mut()
            }
            
            if !self.isRed((*pointer).right_child) && 
                !(*pointer).right_child.is_null() &&
                !self.isRed((*(*pointer).right_child).left_child) 
            {
                pointer = self.moveRedRight(pointer);
            }

            // Recurse into right child
            (*pointer).right_child = self._deleteMax((*pointer).right_child);

            // Fix-up on way-up
            self.fixUp(pointer)
        }
    }

    fn delete(&mut self, key: T) {
        if self.root.is_null() {return}
        unsafe {
            self.root = self._delete(self.root, key);
            if self.root.is_null() {return}
            (*self.root).colour = Colour::Black;
        }
    }

    // Rotate left-leaning red links to the right on the search path
    // If node to be delete is internal node, replace key and value with those in the minimum node on the right subtree
    // Then delete the minimum in the right subtree

    fn _delete(&mut self, mut pointer: *mut Node<T, U>, key: T) -> *mut Node<T, U> {
        // if pointer.is_null() {panic!("Could not find node to be deleted")}
        if pointer.is_null() {return ptr::null_mut()}

        unsafe {
            // If < current key
            if key < (*pointer).key {

                // Maintain invariant that there is a red left child
                if ( !self.isRed((*pointer).left_child) && (*pointer).left_child.is_null() ) ||
                ( !self.isRed((*pointer).left_child) && !self.isRed((*(*pointer).left_child).left_child) ) {
                    pointer = self.moveRedLeft(pointer);
                }

                // Recurse into left child
                (*pointer).left_child = self._delete((*pointer).left_child, key);
            // Else if >= current key
            } else {
                // Ensure there is a red right child
                if self.isRed((*pointer).left_child) {
                    pointer = self.rotateRight(pointer);
                }

                // Found key, and no further right child => Happy case of just deleting this node
                if key == (*pointer).key && (*pointer).right_child.is_null() {
                    let _removed_node = Box::from_raw(pointer); // Free deleted node, prevent memory leak
                    return ptr::null_mut()
                }

                // Make sure red right child
                if !self.isRed((*pointer).right_child) && 
                    !(*pointer).right_child.is_null() &&
                    !self.isRed((*(*pointer).right_child).left_child) 
                {
                    pointer = self.moveRedRight(pointer);
                }

                if key == (*pointer).key {
                    // Replace current node with min of right subchild
                    // Clone to create new variables to represent minimum key and minimum value
                    // Otherwise deleteMin will remove the node, and we will be referencing a removed node
                    let right_subchild_min_key = (*self._min((*pointer).right_child)).key.clone();
                    let right_subchild_min_value = self.get(right_subchild_min_key).clone().unwrap();
                    (*pointer).key = right_subchild_min_key;
                    (*pointer).value = right_subchild_min_value;

                    // Remove the minimum in the right subtree
                    (*pointer).right_child = self._deleteMin((*pointer).right_child)
                } else {
                    (*pointer).right_child = self._delete((*pointer).right_child, key);
                }
            }

        }

        self.fixUp(pointer)
    }


    fn printTree(&self) {
        let mut tree_vec: Vec<(T, usize, Colour)> = Vec::new();
        self._printTree(&mut tree_vec, self.root, 0);

        let mut max_level = 0;

        for node in tree_vec.clone() {
            if node.1 > max_level {max_level = node.1}
        }

        for i in 0..max_level + 1{
            for node in tree_vec.clone() {
                if node.1 == i {
                    println!("{:?}", node);
                }
            }
        }
    }

    fn _printTree(&self, vec: &mut Vec<(T, usize, Colour)>, pointer: *mut Node<T, U>, level: usize) {
        if pointer.is_null() {return}
        unsafe {
            vec.push(((*pointer).key, level, (*pointer).colour));
            self._printTree(vec, (*pointer).left_child, level + 1);
            self._printTree(vec, (*pointer).right_child, level + 1);
        }
    }

}

impl<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> Drop for RedBlackBST<T, U> {
    fn drop(&mut self) {
        while !self.is_empty() {
            self.deleteMin();
        }
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
    let mut bst = RedBlackBST::new();
    bst.put("E", 0);
    bst.put("A", 1);
    bst.put("S", 2);
    bst.put("Y", 3);
    bst.put("Q", 4);
    bst.put("U", 5);
    bst.put("E", 6);
    bst.put("S", 7);
    bst.put("T", 8);
    bst.put("I", 9);
    bst.put("O", 10);
    bst.put("N", 11);


    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("E"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("S"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("Q"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("N"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("N"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("Z"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.deleteMax());
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.deleteMax());
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("I"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("O"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("A"));
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.delete("T"));
    println!("{:?}", bst.all_keys());
}