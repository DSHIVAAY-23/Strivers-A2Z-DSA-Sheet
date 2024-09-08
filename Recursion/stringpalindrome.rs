fn is_palindrome(s: &mut Vec<char>, i: usize, j: usize) -> bool {
    // Base case: if the indices cross each other, stop recursion
    if i >= j {
        return true;
    }

    // If characters at index i and j don't match, it's not a palindrome
    if s[i] != s[j] {
        return false;
    }

    // Recursively check the next pair of characters
    is_palindrome(s, i + 1, j - 1)
}

fn main() {
    let s = String::from("madam");

    // Convert the string into a Vec<char> for safe indexing
    let mut char_vec: Vec<char> = s.chars().collect();
    let len = char_vec.len();
    
    // Call the is_palindrome function and check if the string is a palindrome
    let result = is_palindrome(&mut char_vec, 0, len - 1);
    
    // Print whether the string is a palindrome or not
    if result {
        println!("The string is a palindrome.");
    } else {
        println!("The string is not a palindrome.");
    }
}
