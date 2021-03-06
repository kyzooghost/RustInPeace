use std::ptr;

struct Node<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> {
    key: T,
    value: U,
    subtree_size: usize,
    left_child: *mut Node<T, U>,
    right_child: *mut Node<T, U>
}

pub struct BinarySearchTree<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> {
    root: *mut Node<T, U>,
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

    pub fn get(&self, key: T) -> Option<U> {
        if self.is_empty() {return None}

        let mut pointer = self.root;

        while !pointer.is_null() {
            unsafe {
                if key < (*pointer).key {pointer = (*pointer).left_child}
                else if key > (*pointer).key {pointer = (*pointer).right_child}
                else {
                    return Some((*pointer).value)
                }
            }
        }

        None
    }

    pub fn put(&mut self, key: T, value: U) {

        #[derive(Clone, Debug)]
        enum Side {
            Left,
            Right
        }

        let mut side_stack: Vec<Side> = Vec::new();
        let mut pointer = self.root;

        // Attempt search
        while !pointer.is_null() {
            unsafe {
                // Search hit -> update and return
                if key == (*pointer).key {
                    (*pointer).value = value;
                    return

                // Traverse down left child
                } else if key < (*pointer).key {
                    pointer = (*pointer).left_child;
                    side_stack.push(Side::Left);

                // Traverse down right child
                } else {
                    pointer = (*pointer).right_child;
                    side_stack.push(Side::Right);
                }
            }
        }
        
        // If search miss, insert node at pointer

        let new_node = Box::into_raw(
            Box::new
                (Node {
                    key: key, 
                    value: value, 
                    subtree_size: 1,
                    left_child: ptr::null_mut(), 
                    right_child: ptr::null_mut()
                })
        );

        // INSERT NEW NODE

        // No prior nodes, simple case of inserting node at root
        if side_stack.is_empty() {
            self.root = new_node;
            return
        }

        // Else need to follow side-stack 
        else {

            pointer = self.root;
            let mut side_stack_clone = side_stack.clone();

            // Follow search path to parent of new_node
            while side_stack_clone.len() > 1 {
                match side_stack_clone.remove(0) {
                    Side::Left => {
                        unsafe {
                            pointer = (*pointer).left_child;
                        }
                    },
                    Side::Right => {
                        unsafe {
                            pointer = (*pointer).right_child;
                        }
                    }
                }
            }

            match side_stack_clone.remove(0) {
                Side::Left => {
                    unsafe {
                        (*pointer).left_child = new_node;
                    }
                },
                Side::Right => {
                    unsafe {
                        (*pointer).right_child = new_node;
                    }
                }
            }

            assert!(side_stack_clone.is_empty(), "side stack should be empty")
        }

        side_stack.pop(); // Inserted node, remove direction to new_node. 

        // UPDATE SUBTREE_COUNT

        while !side_stack.is_empty() {
            pointer = self.root;
            let mut side_stack_clone = side_stack.clone();

            while !side_stack_clone.is_empty() {
                match side_stack_clone.remove(0) {
                    Side::Left => {
                        unsafe {
                            pointer = (*pointer).left_child;
                        }
                    },
                    Side::Right => {
                        unsafe {
                            pointer = (*pointer).right_child;
                        }
                    }
                }
            }

            unsafe {
                (*pointer).subtree_size = self._subtree_size((*pointer).left_child) + self._subtree_size((*pointer).right_child) + 1;     
            }
            
            side_stack.pop();
        }

        // Update root
        unsafe {
            (*self.root).subtree_size = self._subtree_size((*self.root).left_child) + self._subtree_size((*self.root).right_child) + 1; 
        }    
    }

    pub fn min(&self) -> Option<T> {
        if self.is_empty() {return None}

        let mut pointer = self.root;

        unsafe {
            while !(*pointer).left_child.is_null() {
                pointer = (*pointer).left_child;
            }

            return Some ((*pointer).key)
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

        let mut pointer = self.root;

        unsafe {
            while !(*pointer).right_child.is_null() {
                pointer = (*pointer).right_child;
            }

            return Some ((*pointer).key)
        }
    }

    pub fn floor(&self, key: T) -> Option<T> {
        if self.size() == 0 {return None}

        let mut current_floor = self.root;
        let mut pointer = self.root;
        let mut in_right_subtree_search = false;

        while !pointer.is_null() {
            unsafe {

                // Search hit
                if key == (*pointer).key {
                    return Some(key)
                }

                // If key < pointer key, search down left child
                else if key < (*pointer).key {
                    // If not in right_subtree_search, floor must be on the left
                    if !in_right_subtree_search {
                        current_floor = (*pointer).left_child;
                        pointer = (*pointer).left_child;

                    // Else in right_subtree_search
                    } else {
                        // Found new floor
                        if !(*pointer).left_child.is_null() && (*(*pointer).left_child).key < key {
                            current_floor = (*pointer).left_child;
                            in_right_subtree_search = false;
                        }
                        pointer = (*pointer).left_child;
                    }
                }

                // If key > pointer key, search down right child
                // Floor could be in left
                else {
                    in_right_subtree_search = true;

                    // Found new floor
                    if !(*pointer).right_child.is_null() && (*(*pointer).right_child).key < key {
                        current_floor = (*pointer).right_child;
                        in_right_subtree_search = false;
                    }   

                    pointer = (*pointer).right_child;
                }
            }
        }

        unsafe {
            if current_floor.is_null() {return None}
            else {return Some((*current_floor).key)}
        }
    }

    pub fn ceiling(&self, key: T) -> Option<T> {
        if self.size() == 0 {return None}

        let mut current_ceiling = self.root;
        let mut pointer = self.root;
        let mut in_left_subtree_search = false;

        while !pointer.is_null() {
            unsafe {

                // Search hit
                if key == (*pointer).key {
                    return Some(key)
                }

                // If key > pointer key, search down right child
                else if key > (*pointer).key {
                    // If not in left_subtree_search, floor must be on the right
                    if !in_left_subtree_search {
                        current_ceiling = (*pointer).right_child;
                        pointer = (*pointer).right_child;

                    // Else in left_subtree_search
                    } else {
                        // Found new ceiling
                        if !(*pointer).right_child.is_null() && (*(*pointer).right_child).key > key {
                            current_ceiling = (*pointer).right_child;
                            in_left_subtree_search = false;
                        }
                        pointer = (*pointer).right_child;
                    }
                }

                // If key < pointer key, search down left child
                // Floor could be in right
                else {
                    in_left_subtree_search = true;

                    // Found new ceiling
                    if !(*pointer).left_child.is_null() && (*(*pointer).left_child).key > key {
                        current_ceiling = (*pointer).left_child;
                        in_left_subtree_search = false;
                    }   

                    pointer = (*pointer).left_child;
                }
            }
        }

        unsafe {
            if current_ceiling.is_null() {return None}
            else {return Some((*current_ceiling).key)}
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

    pub fn deleteMax(&mut self) {
        assert!(!self.is_empty(), "cannot delete from empty tree");
        self.root = self._deleteMax(self.root);
    }

    fn _deleteMax(&self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        unsafe {
            if (*node_pointer).right_child.is_null() {
                let left_child = (*node_pointer).left_child;
                let _removed_node = Box::from_raw(node_pointer); // Free deleted node, prevent memory leak
                return left_child
            }
            (*node_pointer).right_child = self._deleteMax((*node_pointer).right_child);
            (*node_pointer).subtree_size = 1 + self._subtree_size((*node_pointer).left_child) + self._subtree_size((*node_pointer).right_child);
            return node_pointer
        }
    }

    pub fn deleteMin(&mut self) {
        assert!(!self.is_empty(), "cannot delete from empty tree");
        self.root = self._deleteMin(self.root);
    }

    // Need to explicitly free the deleted node or else memory leak
    fn _deleteMin(&self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {
        
        unsafe {
            // Base-case, can no-longer recurse into left child
            // Need to free the deleted node here
            if (*node_pointer).left_child.is_null() {
                let right_child = (*node_pointer).right_child;
                let _removed_node = Box::from_raw(node_pointer); // Free deleted node
                return right_child
            }
            
            // Recurse into left child
            (*node_pointer).left_child = self._deleteMin((*node_pointer).left_child);

            // Updating size on the way-up, fine
            (*node_pointer).subtree_size = 1 + self._subtree_size((*node_pointer).left_child) + self._subtree_size((*node_pointer).right_child);

            return node_pointer
        }
    }

    // _deleteMin() function without explicit free of removed node, for use in _delete() function
    // to avoid deref freed pointer (we freed the successor in _deleteMin, but we continue deref-fing 
    // it afterwards in _delete)
    fn _deleteMinForDelete(&self, node_pointer: *mut Node<T, U>) -> *mut Node<T, U> {

        unsafe {
            // Base-case, can no-longer recurse into left child
            // Need to free the deleted node here
            if (*node_pointer).left_child.is_null() {
                let right_child = (*node_pointer).right_child;
                return right_child
            }
            
            // Recurse into left child
            (*node_pointer).left_child = self._deleteMinForDelete((*node_pointer).left_child);

            // Updating size on the way-up, fine
            (*node_pointer).subtree_size = 1 + self._subtree_size((*node_pointer).left_child) + self._subtree_size((*node_pointer).right_child);

            return node_pointer
        }
    }

    pub fn delete(&mut self, key: T) {
        assert!(!self.is_empty(), "cannot delete from empty tree");
        self.root = self._delete(self.root, key);
    }

    // Hibbard eager deletion
    // Delete node X, by replacing it with its successor - smallest key in right subtree
    // Preserve order because no key between x.key and successor.key

    // 1. Save link to node to be deleted to `t`
    // 2. Set `x` to point to successor
    // 3. Set `x.right_child` to deleteMin(t.right) - BST containing all keys larger than x.key after deletion
    // 4. Set `x.left_child` to `t.left` - (all keys less than deleted key)

    fn _delete(&self, mut node_pointer: *mut Node<T, U>, key: T) -> *mut Node<T, U> {
        // Protect against dereferencing null-pointer
        if node_pointer.is_null() {return ptr::null_mut()}

        unsafe {
            // If less than pointer key, recurse into left child
            // Can't do early return, or else miss updating subtree_size on way up
            if key < (*node_pointer).key {
                (*node_pointer).left_child = self._delete((*node_pointer).left_child, key)
            }

            // Else if more than pointer key, recurse into right child
            else if key > (*node_pointer).key {
                (*node_pointer).right_child = self._delete((*node_pointer).right_child, key)
            }

            // Else search hit, and we implement Hibbard eager deletion
            else { 
                if (*node_pointer).right_child.is_null() {
                    let left_child = (*node_pointer).left_child;
                    let _node_to_be_deleted = Box::from_raw(node_pointer);
                    return left_child
                }

                if (*node_pointer).left_child.is_null() {
                    let right_child = (*node_pointer).right_child;
                    let _node_to_be_deleted = Box::from_raw(node_pointer);
                    return right_child
                }

                // Find the successor
                // let successor = self._min((*node_to_be_deleted).right_child);
                let successor = self._min((*node_pointer).right_child);

                // Re-assign successor right child (will find min of node_to_be_deleted subtree - node_to_be_deleted - successor)
                (*successor).right_child = self._deleteMinForDelete((*node_pointer).right_child);

                // Re-assign successor left child
                (*successor).left_child = (*node_pointer).left_child;

                let _node_to_be_deleted = Box::from_raw(node_pointer);

                node_pointer = successor;
            }

            // Update subtree_size
            (*node_pointer).subtree_size = 1 + self._subtree_size((*node_pointer).left_child) + self._subtree_size((*node_pointer).right_child);

            // Return pointer to successor
            return node_pointer
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

    fn _height(&self, pointer: *mut Node<T, U>, accum_height: usize) -> usize {
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