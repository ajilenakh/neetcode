use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map = HashMap::new();

        for word in strs {
            let sorted_word: String = {
                let mut c: Vec<char> = word.chars().collect();
                c.sort();
                c.into_iter().collect()
            };

            if !anagram_map.contains_key(&sorted_word) {
                anagram_map.insert(sorted_word, vec![word.to_string()]); 
            } else {
                anagram_map
                    .get_mut(&sorted_word)
                    .unwrap()
                    .push(word.to_string())
            }
        }

        let output: Vec<Vec<_>> = anagram_map
            .into_iter()
            .map(|(_key, values)| values)
            .collect();

        return output
    }
}