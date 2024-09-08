fn reverse(s: &mut Vec<char>, i: usize, j: usize) {
    // Base case: if the indices cross each other, stop recursion
    if i >= j {
        return;
    }

    // Swap the elements at indices i and j
    s.swap(i, j);

    // Recurse with the next indices
    reverse(s, i + 1, j - 1);
}

fn main() {
    let mut s = String::from("hello");

    // Convert the string into a Vec<char> for safe indexing and swapping
    let mut char_vec: Vec<char> = s.chars().collect();
    let len = char_vec.len();
    
    // Call the reverse function
    reverse(&mut char_vec, 0, len - 1);
    
    // Convert Vec<char> back to a string
    let reversed_string: String = char_vec.into_iter().collect();

    println!("{}", reversed_string); // Output: olleh
}
