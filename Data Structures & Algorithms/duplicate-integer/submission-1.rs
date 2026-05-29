use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::new(); 
        
        // Loop through each number in the input vector
        for n in nums {
            if !seen.insert(n) {
                return true;
            }
        }

        // If we finish the loop without finding any duplicates,
        false
    }
}
