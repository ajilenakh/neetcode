use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        // HashMap to store: number -> index
        // This lets us quickly find if a needed complement exists
        let mut hash_nums: HashMap<i32, i32> = HashMap::new();

        // First pass: store every number with its index in the HashMap
        // Example: nums = [2, 7, 11, 15]
        // becomes: {2:0, 7:1, 11:2, 15:3}
        for (i, num) in nums.iter().enumerate() {
            hash_nums.insert(*num, i as i32);
        }

        // Second pass: for each number, check if its complement exists
        for (i, num) in nums.iter().enumerate() {

            // The value we need to reach the target sum
            let difference = target - num;

            // Check:
            // 1. Does the complement exist in the HashMap?
            // 2. Is it NOT the same element (avoid using same index twice)?
            if hash_nums.contains_key(&difference)
                && *hash_nums.get(&difference).unwrap() != i as i32
            {
                // If found, return current index and complement index
                return vec![i as i32, *hash_nums.get(&difference).unwrap()];
            }
        }

        // If no valid pair is found, return empty vector
        vec![]
    }
}