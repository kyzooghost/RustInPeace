#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 3.2.30

// Order check. Write a recursive method isOrdered() that takes a Node and two keys min and max as arguments and returns true if all the keys in the tree are between min and max; min and max are indeed the smallest and largest keys in the tree, respec- tively; and the BST ordering property holds for all keys in the tree; false otherwise.

// This is the same thing as keys() method, but you input arbitrary Node instead of root into self._keys().
// You get a range of keys from this method, then you check that the this range is within your provided min and max arguments


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
    println!("{:?}", bst.get("E"));
    println!("{:?}", bst.get("A"));
    println!("{:?}", bst.get("S"));
    println!("{:?}", bst.get("Y"));
    println!("{:?}", bst.get("Q"));
    println!("{:?}", bst.get("U"));
    println!("{:?}", bst.get("E"));
    println!("{:?}", bst.get("S"));
    println!("{:?}", bst.get("T"));
    println!("{:?}", bst.get("I"));
    println!("{:?}", bst.get("O"));
    println!("{:?}", bst.get("N"));
    println!("{:?}", bst.min());
    println!("{:?}", bst.max());
    println!("{:?}", bst.size());
    println!("------", );
    println!("{:?}", bst.all_keys());
    println!("{:?}", bst.isBinaryTree(bst.root));

}