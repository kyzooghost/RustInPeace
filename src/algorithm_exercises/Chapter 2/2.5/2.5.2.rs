#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// p366 Exercises

// 2.5.2

// Write a program that reads a list of words from standard input and prints all two-word compound words in the list. 
// For example, if after, thought, and afterthought are in the list, then afterthought is a compound word.

// O(N) - make sure all lower case (or higher case)
// Sort the list - O(N log N)
// A compound word must start with a smaller word
// Let array k[] be the sorted array of words
// Every word k[n+...] which includes k[n], is a possible compound word

// Create hashtable, key = definite non-compound, value = possible compounds
// O(N^2) worst case process - if k[n+1] includes k[n], 
// then k[n+1] is a possible compound and we should also check k[n+2] with k[n]
// We also need to compare k[n+2] with k[n+1]

// O (N^2)
// For each possible compound, find right substring with original non-compound subtracted
// And binary search for the right substring
// If match, add to array of confirmed compound words

fn main() {
    use std::collections::HashMap;
    
    let non_capital_checked_string_vec = vec!["begin", "end", "test", "afterthought", "Brazil", "thought", "after", "beginend", "abc" ];

    let mut capital_checked_string_vec: Vec<String> = Vec::new();
    // Had to use Vec<String> because not allowed to create temporary .to_lower_case() variable, 
    // The push frees it, but the capital_checked_string vec is still borrowing it
    let mut word_to_potential_compounds: HashMap<String, Vec<String>> = HashMap::new();
    let mut confirmed_compound_words: Vec<String> = Vec::new();

    for element in non_capital_checked_string_vec {capital_checked_string_vec.push(element.to_lowercase())}

    // Sort O(N log N)
    capital_checked_string_vec.sort();

    // O(N) to create new hashmap from vector for O(1) lookup later on
    let mut capital_checked_string_hashmap: HashMap<String, bool> = HashMap::new();
    for element in capital_checked_string_vec.clone() {capital_checked_string_hashmap.insert(element, true);}

    // O(N^2) worst case
    for (index, element) in capital_checked_string_vec.iter().enumerate() {
        let mut i = 1;

        while index + i < capital_checked_string_vec.len() && capital_checked_string_vec[index + i].contains(element) {
            if word_to_potential_compounds.contains_key(element) {
                let compound_words_array = word_to_potential_compounds.get_mut(element).unwrap();
                compound_words_array.push(capital_checked_string_vec[index + i].clone());
            } else {
                word_to_potential_compounds.insert(element.to_string(), vec![capital_checked_string_vec[index + i].clone()]);
            }
            i += 1;
        }
    }

    // Iterate through each potential compound word - O(N^2 ln N) worst case - because compound words can have
    // multiple different starts

    // Actually you can improve this from O(N^2 ln N) to O(N^2)
    // Instead of doing a binary search for the right substring, create a hashmap and make it an O(1) lookup.
    // Basically trading space efficiency for time efficiency

    for (key, compound_vec) in word_to_potential_compounds.iter() {
        if compound_vec.len() > 0 {
            for compound_word in compound_vec {
                let key_bytes = key.len(); // 1 byte for each character that is first 128 Unicode
                let right_substring = &compound_word[key_bytes..]; // Right substring
                
                // Binary search for right substring, 
                match capital_checked_string_hashmap.contains_key(&right_substring.to_string()) {
                    true => {confirmed_compound_words.push(compound_word.to_string());},
                    false => {}
                }
            }
        }
    }

    println!("{:?}", confirmed_compound_words);
}