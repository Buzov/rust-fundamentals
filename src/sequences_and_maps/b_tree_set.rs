use std::collections::BTreeSet;

// In Rust, a BTreeSet is a sorted set based on a B-tree, which ensures that the elements are kept
// in order and provides efficient operations for insertion, deletion, and lookup. It's part of
// the standard library and is useful when you need a collection of unique items that are sorted.
pub fn test_b_tree_set() {
    let mut btree_set = BTreeSet::new();

    // Inserting elements into the BTreeSet
    btree_set.insert(5);
    btree_set.insert(1);
    btree_set.insert(3);
    btree_set.insert(2);
    btree_set.insert(4);

    // Attempting to insert a duplicate value (no effect)
    btree_set.insert(3);

    // Printing the BTreeSet (elements will be sorted)
    println!("BTreeSet: {:?}", btree_set);

    // Checking if a value exists
    if btree_set.contains(&3) {
        println!("BTreeSet contains 3");
    }

    // Removing an element
    btree_set.remove(&2);

    println!("After removing 2: {:?}", btree_set);

    // Iterating through the BTreeSet
    for value in &btree_set {
        println!("Value: {}", value);
    }

    // Getting the first and last elements
    if let Some(first) = btree_set.iter().next() {
        println!("First element: {}", first);
    }
    if let Some(last) = btree_set.iter().next_back() {
        println!("Last element: {}", last);
    }
}