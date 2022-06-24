#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// Suppose that a certain BST has keys that are integers between 1 and 10, 
// and we search for 5. Which sequence below cannot be the sequence of keys examined?

// a. 10, 9, 8, 7, 6, 5
// b. 4, 10, 8, 7, 5, 3
// c. 1, 10, 2, 9, 3, 8, 4, 7, 6, 5 
// d. 2, 7, 3, 8, 4, 5
// e. 1, 2, 10, 4, 8, 5

mod utils {pub mod BinarySearchTree;}
use utils::BinarySearchTree::BinarySearchTree as BinarySearchTree;

fn main() {
    let mut bst = BinarySearchTree::new();

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
    println!("{:?}", bst.height());
}
