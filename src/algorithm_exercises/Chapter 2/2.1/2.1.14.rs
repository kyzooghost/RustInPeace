#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p249 - Selection sort
// p251 - Insertion sort
// p259 - Shell sort
// p265 - Exercises

// 2.1.14

// Dequeue sort. 

// Explain how you would sort a deck of cards, with the restriction 
// that the only allowed operations are to look at the values of the top two cards, 
// to exchange the top two cards, and to move the top card to the bottom of the deck.

// Ok this is more interesting
// You can only look at the top two cards, exchange them, or move the top to the bottom
// I'm assuming you want the lowest rank at the bottom, and top rank at the top

// You can get stuck in a loop of only exchanging the top two cards
// You're likely going to have to run through the entire deck multiple times
// All you can really do in this situation, is exchange the top two cards if they are out of order, then move the top to the bottom
// There are like no other options

fn main() {
    let vec = vec![35, 39, 49, 75, 89, 19, 78, 85, 18, 84];
    println!("{:?}", "B" < "A");

    let vec = vec!["E", "A", "S", "Y", "S", "H", "E", "L", "L", "S", "O", "R", "T", "Q", "U", "E", "S", "T", "I", "O", "N"];
    let sorted_vec = shellSort(vec);
    println!("{:?}", sorted_vec);
}

fn shellSort(mut vec: Vec<&str>) -> Vec<&str> {
    let N = vec.len();
    let mut swapped = false;
    let mut h = 1;
    while h < N / 3 {h = 3*h + 1}

    while h >= 1 {
        for i in 0..N {
            let mut j = i;
            swapped = false;
    
            while j >= h && vec[j] < vec[j - h] {
                vec.swap(j, j-h);
                j = j - h;
                swapped = true;
            }

            if swapped {
                println!("i: {:?}, j: {:?}, h: {:?}", i, j, h);
                println!("{:?}", vec);
            }
        }
        
        h = h / 3;
    }

    vec
}