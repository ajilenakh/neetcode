use std::str::from_utf8;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut output = String::with_capacity(strs.len() * 10);

        for s in &strs{
            let size_of_str = s.len();
            output.push_str(&format!("{}|{}", size_of_str, s));
        }
        output
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        let mut i = 0;
        let bytes = s.as_bytes();

        while i < s.len() {
            let mut j = i;
            
            while bytes[j] != 124 { // "|" is 124 in ascii
                j += 1;
            }

            let len_str = from_utf8(&bytes[i..j]).unwrap();
            let length: usize = len_str.parse().unwrap();


            let word_start = j+1;
            let word_end = word_start + length;
            let word = from_utf8(&bytes[word_start..word_end]).unwrap();

            output.push(word.to_string());

            i = word_end;
        }
        output
    }
}
