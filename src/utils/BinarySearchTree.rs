use std::ptr;

pub struct Node<T: Clone + PartialOrd + PartialEq + Copy + std::fmt::Debug, U: Clone + PartialOrd + PartialEq + Copy> {
    key: T,
    value: U,
    subtree_size: usize,
    left_child: *mut Node<T, U>,
    right_child: *mut Node<T, U>
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
                        right_child: ptr::null_mut()
                    })
            );

            return new_node
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

    pub fn isBinaryTree(&self, pointer: *mut Node<T, U>) -> bool {
        // Check self, and left and right children

        if pointer.is_null() {return true}

        unsafe {
            let current_node_status = self._subtree_size(pointer) == 1 + self._subtree_size((*pointer).left_child) + self._subtree_size((*pointer).right_child);

            let left_child_status = self.isBinaryTree((*pointer).left_child);

            let right_child_status = self.isBinaryTree((*pointer).right_child);

            current_node_status && left_child_status && right_child_status
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