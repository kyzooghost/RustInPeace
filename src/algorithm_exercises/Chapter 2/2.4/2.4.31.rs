#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p332 Exercises

// 2.4.31 

// Develop a compare-based implementation of the MinPQ API 
// such that insert uses ~ log log N compares 
// and delete the minimum uses ~2 log N compares. 

// Hint : Use binary search on parent pointers to find the ancestor in swim().
// Again, the edge case are a pain in the ass here

pub struct MinPQ<T> {
    heap: Vec<Option<T>>,
    size: usize
}

impl<T: Copy + Ord + std::fmt::Debug> MinPQ<T> {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn new() -> Self {
        MinPQ { size: 0, heap: vec![None] }
    }

    pub fn isEmpty(&self) -> bool {
        self.size == 0
    }

    pub fn min(&self) -> Option<T> {
        if self.size == 0 {return None}
        self.heap[1]
    }

    pub fn insert(&mut self, element: T) {
        self.size += 1;
        self.heap.push(Some(element));
        self.swim(self.size); // Lagging step
    }

    pub fn delMin(&mut self) -> Option<T> {
        if self.size == 0 {return None}

        self.heap.swap(1, self.size);
        let max = self.heap.pop().unwrap();
        self.size -= 1;
        self.sink(1); // Lagging step
        max
    }

    fn swim(&mut self, mut index: usize) {
        // Worst case log N compares (height of binary tree)
        // Log log N compares => Binary search, with N = height of binary tree elements
        
        // Create array with all 'swimmable' node values
        // Binary search within it, swap 'index' with the found node
        // & shuffle all nodes on the way, down one

        let temp = self.heap[index].unwrap();
        let temp_index = index;

        // Don't need to sort it, as before pushing the new element it's already sorted
        // Collect all swimmable parent nodes, with heap index
        let mut swimmable_nodes: Vec<(usize, T)> = Vec::new();

        index /= 2; // Don't need edge case check here, accounting for by while loop condition
        while index > 0 {
            swimmable_nodes.push( (index, self.heap[index].unwrap() ) );
            index /= 2;
        }

        // Find index to swap
        let desired_index = binary_search(temp, &mut swimmable_nodes);

        match desired_index {
            None => return,
            Some(desired_index) => {
                // Before desired_index, move all nodes down one level to child
                index = temp_index; // Reset index
                while index > desired_index {
                    self.heap[index] = self.heap[index / 2];
                    index /= 2
                }

                // Insert into desired_index
                self.heap[desired_index] = Some(temp);
            }
        }

        // while index > 1 && self.heap[index / 2] > self.heap[index] {
        //     self.heap.swap(index, index / 2);
        //     index = index / 2;
        // }
    }

    fn sink(&mut self, mut index: usize) {
        // 2 log N compares already
        while 2*index <= self.size {
            let mut j = 2 * index;
            if j < self.size && self.heap[j] > self.heap[j + 1] {j += 1}
            if self.heap[index] <= self.heap[j] {break}
            self.heap.swap(index, j);
            index = j;
        }   
    }
}

// This is the API or external function, made as simple as possible
// But internally, need to keep entire array in recursive calls
fn binary_search<T: Copy + Ord + std::fmt::Debug>(elem: T, array: &mut [(usize, T)]) -> Option<usize> {
    let length = array.len();
    if length == 0 {return None}
    return binary_search_internal(elem, array, 0, length - 1);
}

fn binary_search_internal<T: Copy + Ord + std::fmt::Debug>(elem: T, array: &mut [(usize, T)], low: usize, high: usize) -> Option<usize> {
    let length = array.len();

    // Input guard, should never get here
    if low > high {return None}
    // Single element left, need to return a value
    else if low == high {
        // If equal, swap
        if elem == array[low].1 {return Some(array[low].0)}
        // If less than, swap parent node
        else if elem < array[low].1 {return Some(array[low].0)}
        // If more than, swap child node
        else {
            if low == 0 {return None} // Elem is > every parent, no swap needed
            else {return Some(array[low - 1].0)}
        }
    // Multiple elements left, recurse into appropriate half
    } else {
        let mid = (high + low) / 2;
        // If equal, can recurse early
        if elem == array[mid].1 {return Some(array[mid].0)}
        // If smaller, recurse in right half
        else if elem < array[mid].1 {
            if mid == length - 1 {return binary_search_internal(elem, array, mid, high)}
            else { return binary_search_internal(elem, array, mid + 1, high) }
        }
        // If bigger, recurse in left half
        else {
            if mid == 0 { return binary_search_internal(elem, array, low, 0) }
            else {return binary_search_internal(elem, array, low, mid - 1)}
        }
    }
}

fn main() {
    let vec = vec!["E", "A", "S" ,"Y", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let mut pq = MinPQ::new();
    for element in vec {
        pq.insert(element);

    }  
    while pq.size() > 0 {
        println!("{:?}", pq.delMin());
    } 
}

// Much more concise because no need to explicitly deal with the edge cases in a non-recursive solution
// log2 implementation is most interesting, with right and left bitwise shifts for log2, /= 2, *= 2
private int binarySearchToGetTargetAncestor(int index) {
    int higherLevel = 0;
    int indexLevel = log2(index); // What level did we push to?
    int lowerLevel = indexLevel;

    while (lowerLevel != higherLevel) {
        // Get middle level
        int middleLevel = higherLevel + (lowerLevel - higherLevel) / 2;

        // indexLevel - middleLevel == levels to jump = how many times do we halve,
        // to get to parent node
        int parentIndex = index / (1 << (indexLevel - middleLevel));

        numberOfCompares++;

        // If parent node > index
        if (ArrayUtil.more(priorityQueue[parentIndex], priorityQueue[index])) {
            // Search in right half
            lowerLevel = middleLevel;
        } else {
            // Else search in left half
            higherLevel = middleLevel + 1;
        }
    }
    return index / (1 << (indexLevel - higherLevel));
}

private int log2(int value) {
    int result = 0;
    // Use right bitwise shift as /= 2
    // Every instance you can half (with integer type, no decimals), without hitting 0, add 1
    while ((value >>= 1) != 0) {
        result++;
    }
    return result;
}