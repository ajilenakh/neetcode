use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {

        // Early exit:
        // If lengths differ, they cannot be anagrams
        if s.len() != t.len() {
            return false;
        }

        // Build frequency maps and compare them directly
        // We pass &s and &t because freq_map borrows the string
        Self::freq_map(&s) == Self::freq_map(&t)
    }

    // Builds a frequency map of characters in a string
    // Example: "aab" → {a: 2, b: 1}
    fn freq_map(a: &str) -> HashMap<char, usize> {

        let mut hm = HashMap::new();

        // Count each character using a HashMap
        // entry() gives mutable access or inserts default 0
        for c in a.chars() {

        // hm.entry(c)
        // - Checks if character `c` already exists in the HashMap
        // - If it exists → gives access to its stored value
        // - If it does NOT exist → creates a new entry for `c`
        //
        // .or_insert(0)
        // - If `c` is not in the map, insert it with initial value 0
        // - Returns a mutable reference to the value for key `c`
        //
        // * (dereference)
        // - Converts the mutable reference (&mut usize) into the actual value
        //
        // += 1
        // - Increments the count because we saw this character once more
        *hm.entry(c).or_insert(0) += 1;
        }

        hm
    }
}