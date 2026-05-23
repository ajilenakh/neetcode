use std::collections::HashMap;

impl Solution {

    // Main function: checks if two strings are anagrams
    pub fn is_anagram(s: String, t: String) -> bool {

        // Convert string `s` into a frequency map:
        // example: "aab" -> { 'a': 2, 'b': 1 }
        let hashs = Solution::freq_map(s);

        // Convert string `t` into a frequency map:
        // example: "aba" -> { 'a': 2, 'b': 1 }
        let hasht = Solution::freq_map(t);

        // If both frequency maps are identical, then:
        // - both strings have same characters
        // - with same counts
        // => they are anagrams
        hashs == hasht
    }

    // Helper function: builds a frequency map of characters in a string
    fn freq_map(a: String) -> HashMap<char, usize> {

        // Create an empty HashMap:
        // key   -> character (char)
        // value -> how many times it appears (usize)
        let mut hm: HashMap<char, usize> = HashMap::new();

        // Iterate over each character in the string
        for i in a.chars() {

            // hm.entry(i) -> look for character `i` in the map
            // .or_insert(0) -> if not found, insert it with value 0
            //
            // The result is a mutable reference to the value (usize)
            //
            // * dereferences the value so we can modify it
            // += 1 increases the count for this character
            *hm.entry(i).or_insert(0) += 1;
        }

        // Return the completed frequency map
        hm
    }
}