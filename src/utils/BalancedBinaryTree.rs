use std::ptr;

enum Side {
    Left,
    Right
}

struct Node<T: Clone> {
    elem: T,
    index: usize,
    parent: *mut Node<T>,
    left_child: *mut Node<T>,
    right_child: *mut Node<T>
}

pub struct BalancedBinaryTree<T: Clone> {
    root: *mut Node<T>,
    last_node: *mut Node<T>,
    size: usize
}

pub struct IntoIter<T: Clone>(BalancedBinaryTree<T>);

impl<T: Clone> BalancedBinaryTree<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        BalancedBinaryTree { root: ptr::null_mut(), last_node: ptr::null_mut(), size: 0 }
    }

    pub fn insert(&mut self, elem: T) {
        if self.size == 0 {
            let new_root = Box::into_raw(
                                Box::new
                                    (Node {
                                        elem: elem, 
                                        index: 1,
                                        parent: ptr::null_mut(), 
                                        left_child: ptr::null_mut(), 
                                        right_child: ptr::null_mut()
                                    })
            );

            self.root = new_root;
            self.last_node = new_root;
        } else {
            /*

            Case I: Completed level (size before adding new node = 2^n - 1)
                Go from root, keep traversing down left child until None, then insert left child
                O(lg N)

            Case II: Not completed level && size before add new node = even
                Start from last
                Go to parent
                Insert right child
                O(1)

            Case III: Not completed level && size before add new node = odd
                Start from last
                Go to parent
                Add one to parent index
                Find path to get to node(parent + 1)
                    Apply index / 2 until index == 1 (reached root)
                    If index == odd, push right on the stack, otherwise push left
                    From root, pop the stack and follow direction until it is empty
                    ~2 O(lg N) - effectively going up and then down the binary tree
                Add left child

            */

            let casted_size_plus_one = (self.size + 1) as f64; // Need to cast to f64 to use log2 function

            // If floor completed
            if casted_size_plus_one.log2().fract() == 0.0 {
                unsafe {
                    let mut pointer = self.root; // Start pointer at root

                    // Traverse down left child from root, until reached bottom
                    while !(*pointer).left_child.is_null() {
                        pointer = (*pointer).left_child;
                    }

                    let new_node = Box::into_raw(
                        Box::new
                            (Node {
                                elem: elem,
                                index: self.size + 1,
                                parent: pointer, 
                                left_child: ptr::null_mut(), 
                                right_child: ptr::null_mut()
                            })
                    );

                    (*pointer).left_child = new_node;
                    self.last_node = new_node;
                }
            // Else if floor not completed
            } else {

                // If currently at even size
                if self.size % 2 == 0 {
                    unsafe {
                        let mut pointer = self.last_node; // Start pointer at last node
                        pointer = (*pointer).parent; // Move pointer to parent
                        if !(*pointer).right_child.is_null() {panic!("Should not have right child")}

                        let new_node = Box::into_raw(
                            Box::new
                                (Node {
                                    elem: elem,
                                    index: self.size + 1,
                                    parent: pointer, 
                                    left_child: ptr::null_mut(), 
                                    right_child: ptr::null_mut()
                                })
                        );

                        // Insert new right child
                        (*pointer).right_child = new_node;
                        self.last_node = new_node;
                    }
                // Else if currently at odd size
                } else {
                    unsafe {
                        let mut pointer = self.last_node; // Start pointer at last node
                        pointer = (*pointer).parent; // Move pointer to parent
                        let mut desired_index = (*pointer).index + 1; // Get index of node after parent

                        let mut side_stack:Vec<Side> = Vec::new();

                        // Determine path from desired_index to root
                        while desired_index > 1 {
                            if desired_index % 2 == 0 {side_stack.push(Side::Left)} // If even, is a left child
                            else {side_stack.push(Side::Right)} // Else if odd, is a right child
                            desired_index /= 2; // Go up the binary tree until reached the root
                        }

                        pointer = self.root; // Move pointer to root

                        // Follow path backwards from root to desired_index
                        while !side_stack.is_empty() {
                            match side_stack.pop().unwrap() {
                                Side::Left => {pointer = (*pointer).left_child;},
                                Side::Right => {pointer = (*pointer).right_child;}
                            }
                        }

                        let new_node = Box::into_raw(
                            Box::new
                                (Node {
                                    elem: elem,
                                    index: self.size + 1,
                                    parent: pointer, 
                                    left_child: ptr::null_mut(), 
                                    right_child: ptr::null_mut()
                                })
                        );

                        // Insert new right child
                        (*pointer).left_child = new_node;
                        self.last_node = new_node;
                    }
                }

            }
        } 
        
        self.size += 1;
    } 

    pub fn remove_last_node(&mut self) -> Option<T> {
        if self.size == 0 {return None}
        unsafe {
            // Have to obtain raw pointer from self.last_node, before I put it into a Box
            // Otherwise Miri screams at me lol
            // Invalid to take a raw pointer from one that has been Boxed? But cargo run works fine
            let mut pointer = self.last_node; // Start from last node
            pointer = (*pointer).parent; // Go to parent

            let removed_node = Box::from_raw(self.last_node);

            // Edge case: Single node left
            if self.size == 1 {
                self.root = ptr::null_mut();
                self.last_node = ptr::null_mut();
            } else {
                // Edge case: Removing only node from a level
                if (self.size as f64).log2().fract() == 0.0 {
                    (*pointer).left_child = ptr::null_mut(); // Null out pointer to removed_node
                    pointer = self.root; // Go to root
    
                    // Go down tree until reach bottom level
                    while !(*pointer).right_child.is_null() {
                        pointer = (*pointer).right_child;
                    }
    
                    self.last_node = pointer;
                // If even node
                } else if self.size % 2 == 0 {
                    (*pointer).left_child = ptr::null_mut(); // Null out pointer to removed_node
                    let mut desired_index = (*pointer).index - 1; // Get index of node before parent
                    let mut side_stack:Vec<Side> = Vec::new();

                    // Determine path from desired_index to root
                    while desired_index > 1 {
                        if desired_index % 2 == 0 {side_stack.push(Side::Left)} // If even, is a left child
                        else {side_stack.push(Side::Right)} // Else if odd, is a right child
                        desired_index /= 2; // Go up the binary tree until reached the root
                    }

                    pointer = self.root; // Move pointer to root

                    // Follow path backwards from root to desired_index
                    while !side_stack.is_empty() {
                        match side_stack.pop().unwrap() {
                            Side::Left => {pointer = (*pointer).left_child;},
                            Side::Right => {pointer = (*pointer).right_child;}
                        }
                    }

                    self.last_node = (*pointer).right_child;
                // If odd node, happy case
                } else {
                    (*pointer).right_child = ptr::null_mut(); // Null out pointer to removed_node
                    self.last_node = (*pointer).left_child;
                }
            }

            self.size = self.size - 1;
            Some(removed_node.elem)
        }
    }

    pub fn peek_at_root(&self) -> Option<&T> {
        if self.size == 0 {return None}
        unsafe {
            self.root.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut_at_root(&mut self) -> Option<&mut T> {
        if self.size == 0 {return None}
        unsafe {
            self.root.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn peek_at_last_node(&self) -> Option<&T> {
        if self.size == 0 {return None}
        unsafe {
            self.last_node.as_ref().map(|node| &node.elem)
        }
    }

    pub fn peek_mut_at_last_node(&mut self) -> Option<&mut T> {
        if self.size == 0 {return None}
        unsafe {
            self.last_node.as_mut().map(|node| &mut node.elem)
        }
    }

    pub fn peek_at_index(&self, mut index: usize) -> Option<&T> {
        if index == 0 || index > self.size() {return None}
        if self.size == 0 {return None}
        else if index == 1 {return self.peek_at_root()}
        else if index == self.size() {return self.peek_at_last_node()}
        else {
            unsafe {
                let mut side_stack:Vec<Side> = Vec::new();

                // Determine path from desired_index to root
                while index > 1 {
                    if index % 2 == 0 {side_stack.push(Side::Left)} // If even, is a left child
                    else {side_stack.push(Side::Right)} // Else if odd, is a right child
                    index /= 2; // Go up the binary tree until reached the root
                }

                let mut pointer = self.root;

                // Follow path backwards from root to desired_index
                while !side_stack.is_empty() {
                    match side_stack.pop().unwrap() {
                        Side::Left => {pointer = (*pointer).left_child;},
                        Side::Right => {pointer = (*pointer).right_child;}
                    }
                }

                pointer.as_ref().map(|node| &node.elem)
            }
        }
    }

    pub fn peek_mut_at_index(&mut self, mut index: usize) -> Option<&mut T> {
        if index == 0 || index > self.size() {return None}
        if self.size == 0 {return None}
        else if index == 1 {return self.peek_mut_at_root()}
        else if index == self.size() {return self.peek_mut_at_last_node()}
        else {
            unsafe {
                let mut side_stack:Vec<Side> = Vec::new();

                // Determine path from desired_index to root
                while index > 1 {
                    if index % 2 == 0 {side_stack.push(Side::Left)} // If even, is a left child
                    else {side_stack.push(Side::Right)} // Else if odd, is a right child
                    index /= 2; // Go up the binary tree until reached the root
                }

                let mut pointer = self.root;

                // Follow path backwards from root to desired_index
                while !side_stack.is_empty() {
                    match side_stack.pop().unwrap() {
                        Side::Left => {pointer = (*pointer).left_child;},
                        Side::Right => {pointer = (*pointer).right_child;}
                    }
                }

                pointer.as_mut().map(|node| &mut node.elem)
            }
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // Require Clone trait for this method to work
    pub fn swap(&mut self, x: usize, y: usize) {
        if x > self.size || y > self.size {panic!("indexes > size")}
        let x_value = self.peek_at_index(x).unwrap().clone();
        let y_value = self.peek_at_index(y).unwrap().clone();
        let x_mut_ref = self.peek_mut_at_index(x).unwrap();
        *x_mut_ref = y_value;
        let y_mut_ref = self.peek_mut_at_index(y).unwrap();
        *y_mut_ref = x_value;
    }
}

impl<T: Clone> Drop for BalancedBinaryTree<T> {
    fn drop(&mut self) {
        while let Some(_) = self.remove_last_node() { }
    }
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove_last_node()
    }
}