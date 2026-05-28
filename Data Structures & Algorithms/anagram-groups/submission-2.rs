use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map: HashMap<[i32; 26], Vec<String>> = HashMap::new();

        for word in strs {
            // Create a 26-letter frequency signature for the word.
            // Maps 'a' -> 0, 'b' -> 1, ..., 'z' -> 25 by exploiting utilizing ASCII byte values.
            let mut frequency_char = [0i32; 26];
            for &b in word.as_bytes() {
                frequency_char[(b - b'a') as usize] += 1
            };

            anagram_map
                .entry(frequency_char)
                .or_default()
                .push(word);
        }

        let output: Vec<Vec<_>> = anagram_map
            .into_iter()
            .map(|(_key, values)| values)
            .collect();

        return output
    }
}