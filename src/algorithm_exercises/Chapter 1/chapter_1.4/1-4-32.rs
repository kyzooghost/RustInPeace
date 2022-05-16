#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// 1.4.32

// Amortized analysis. 

// Prove that, starting from an empty stack, 
// the number of array accesses used by any sequence of M operations 
// in the resizing array implementation of Stack is proportional to M.

// Prove that the # of array accesses, is proportional to the number of operations

public class ResizingArrayStack<Item> implements Iterable<Item> {

    private static final int INIT_CAPACITY = 8;
    private Item[] a;         // array of items
    private int n;            // number of elements on stack

    // Constructor
    public ResizingArrayStack() {
        a = (Item[]) new Object[INIT_CAPACITY];
        n = 0;
    }

    public boolean isEmpty() {
        return n == 0;
    }

    public int size() {
        return n;
    }

    // resize the underlying array holding the elements
    private void resize(int capacity) {
        assert capacity >= n;

        Item[] copy = (Item[]) new Object[capacity];

        // Manually copy each element to new array
        for (int i = 0; i < n; i++) {
            copy[i] = a[i];
        }

        // Change pointer value
        a = copy;
    }

    // If no resize, 1 array access
    // Otherwise, 'size + 1' array accesses
    public void push(Item item) {
        if (n == a.length) resize(2*a.length);    // double size of array if necessary
        a[n++] = item;                            // add item
    }

    // If no resize, 1 array access
    // Otherwise `size + 1` array accesses
    public Item pop() {
        if (isEmpty()) throw new NoSuchElementException("Stack underflow");
        
        Item item = a[n-1];
        a[n-1] = null;                              // to avoid loitering
        n--;

        // shrink size of array if necessary
        if (n > 0 && n == a.length/4) resize(a.length/2);
        return item;
    }
}

// If just push, # of array accesses = (# of push) + 
// Every (INIT_CAPACITY) * 2^n, where n is an integer, add (INIT_CAPACITY) * 2^n
// To get to INIT_CAPACITY, need INIT_CAPACITY of pushes
// So at n = 0, # of array accesses = (# of push) + (# of push)
// At n = 1 => (# of push) + (# of push) + (# of push) * 2
// At n = 2 => (# of push) + (# of push) + (# of push) * 2^1 + (# of push) * 2^2
// => (# of push)(1 + 2^0 + 2^1 + 2^2 + ...)

// Similar logic for pop()
// Resize every time you hit a halving

fn main() {
}