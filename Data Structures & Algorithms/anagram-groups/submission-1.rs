use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

        for word in strs {
            let sorted_word: String = {
                let mut c: Vec<char> = word.chars().collect();
                c.sort();
                c.into_iter().collect()
            };

            anagram_map
                .entry(sorted_word)
                .or_default()
                .push(word.to_string());
        }

        let output: Vec<Vec<_>> = anagram_map
            .into_iter()
            .map(|(_key, values)| values)
            .collect();

        return output
    }
}