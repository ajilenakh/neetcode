use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // We use a HashMap to store numbers we have already seen.
        // Key   -> the number itself (value from the array)
        // Value -> its index in the array
        let mut hash_nums: HashMap<i32, i32> = HashMap::new();

        // Iterate over the array with both index and value.
        for (i, num) in nums.iter().enumerate() {

            // Compute the value needed to reach the target sum.
            // If num + complement = target, then complement = target - num.
            let compliment = target - num;

            // Check if we have already seen the complement earlier in the array.
            // If it exists in the HashMap, that means we previously encountered
            //
            // We can immediately return the pair of indices.
            if let Some(&j) = hash_nums.get(&compliment) {
                return vec![j, i as i32];
            }

            // If complement is not found, store the current number with its index.
            // This ensures future elements can match against it.
            //
            // We store it *after* the check to avoid using the same element twice.
            hash_nums.insert(*num, i as i32);
        }

        // If no valid pair exists, return an empty vector.
        vec![]
    }
}